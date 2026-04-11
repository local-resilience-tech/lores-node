use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{mpsc, Mutex};

use lores_p2panda::{
    p2panda_core::{identity::PUBLIC_KEY_LEN, Hash, PrivateKey, PublicKey},
    panda_node::{
        LogCount, OperationCountByAuthorAndTopic, PandaNode, PandaPublishError, RequiredNodeParams,
        SubscriptionError,
    },
    IncomingOperation, PandaNodeError, Topic,
};

use crate::api::auth_api::auth_backend::User;

use super::{
    event_encoding::{decode_lores_event, encode_lores_event_payload},
    lores_events::{LoResEvent, LoResEventHeader, LoResEventMetadataV1, LoResEventPayload},
    RegionId,
};

#[derive(Default, Clone)]
pub struct NodeParams {
    pub private_key: Option<PrivateKey>,
    pub network_name: Option<String>,
    pub bootstrap_node_ids: Vec<PublicKey>,
}

#[derive(Debug, Error)]
pub enum PandaContainerError {
    #[error(transparent)]
    PandaNodeError(#[from] PandaNodeError),
}

#[derive(Debug, Error)]
pub enum PandaSubscriptionError {
    #[error(transparent)]
    SubscriptionError(#[from] SubscriptionError),
    #[error("Couldn't get node lock")]
    CouldntGetNodeLock(),
}

#[derive(Clone)]
pub struct PandaContainer {
    params: Arc<Mutex<NodeParams>>,
    node: Arc<Mutex<Option<Arc<PandaNode>>>>,
    lores_events_tx: mpsc::Sender<LoResEvent>,
}

impl PandaContainer {
    pub fn new(events_tx: mpsc::Sender<LoResEvent>) -> Self {
        let params = Arc::new(Mutex::new(NodeParams::default()));

        PandaContainer {
            params,
            node: Arc::new(Mutex::new(None)),
            lores_events_tx: events_tx,
        }
    }

    pub async fn get_params(&self) -> NodeParams {
        let params_lock = self.params.lock().await;
        params_lock.clone()
    }

    pub async fn set_network_name(&self, network_name: String) {
        let mut params_lock = self.params.lock().await;
        params_lock.network_name = Some(network_name);
    }

    pub async fn set_private_key(&self, private_key: PrivateKey) {
        let mut params_lock = self.params.lock().await;
        params_lock.private_key = Some(private_key);
    }

    pub async fn set_bootstrap_node_ids(&self, bootstrap_node_ids: Vec<PublicKey>) {
        let mut params_lock = self.params.lock().await;
        params_lock.bootstrap_node_ids = bootstrap_node_ids;
    }

    pub async fn start(&self, operations_database_url: &str) -> Result<(), PandaContainerError> {
        println!("Starting client");

        let params = self.get_params().await;

        let private_key: Option<PrivateKey> = params.private_key;
        let network_name: Option<String> = params.network_name;
        let boostrap_node_ids: Vec<PublicKey> = params.bootstrap_node_ids;

        if private_key.is_none() {
            println!("P2Panda: No private key found, not starting network");
            return Ok(());
        }

        if network_name.is_none() {
            println!("P2Panda: No network name found, not starting network");
            return Ok(());
        }

        let private_key = private_key.unwrap();
        let network_name = network_name.unwrap();

        self.start_for(
            private_key,
            network_name,
            &boostrap_node_ids,
            operations_database_url,
        )
        .await?;

        Ok(())
    }

    async fn start_for(
        &self,
        private_key: PrivateKey,
        network_name: String,
        boostrap_node_ids: &Vec<PublicKey>,
        operations_database_url: &str,
    ) -> Result<(), PandaNodeError> {
        let required_params = RequiredNodeParams {
            private_key,
            network_id: Hash::new(network_name.as_bytes()),
            bootstrap_node_ids: boostrap_node_ids.clone(),
        };

        let panda_node = Arc::new(PandaNode::new(&required_params, operations_database_url).await?);

        {
            let mut node_lock = self.node.lock().await;
            *node_lock = Some(panda_node);
        }

        println!("P2Panda: Node started. Network name: {}", network_name);

        Ok(())
    }

    pub async fn is_started(&self) -> bool {
        let node = self.node.lock().await;
        node.is_some()
    }

    pub async fn get_public_key(&self) -> Result<PublicKey, Box<dyn std::error::Error>> {
        let params_lock = self.params.lock().await;
        match params_lock.private_key {
            Some(ref key) => Ok(key.public_key()),
            None => Err("Private key not set".into()),
        }
    }

    pub async fn replay_all_regions(&self) -> Result<usize, PandaSubscriptionError> {
        let node_lock = self.node.lock().await;
        let node = match node_lock.as_ref() {
            Some(node) => node.clone(),
            None => return Err(PandaSubscriptionError::CouldntGetNodeLock()),
        };
        drop(node_lock);

        let topics = node.get_subscribed_topics().await;
        let count = topics.len();

        for topic_id in topics {
            let (incoming_tx, mut incoming_rx) = mpsc::channel::<IncomingOperation>(32);
            node.replay_topic(topic_id, incoming_tx).await?;

            let events_tx = self.lores_events_tx.clone();
            tokio::spawn(async move {
                while let Some(incoming) = incoming_rx.recv().await {
                    match Self::decode_incoming_to_lores_event(incoming) {
                        Ok(lores_event) => {
                            if events_tx.send(lores_event).await.is_err() {
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to decode LoResEvent during replay: {}", e);
                        }
                    }
                }
            });
        }

        Ok(count)
    }

    pub async fn join_region(&self, region_id: RegionId) -> Result<Topic, PandaSubscriptionError> {
        let topic_id = Topic::from(<[u8; 32]>::from(region_id));
        self.subscribe(topic_id).await?;
        Ok(topic_id)
    }

    pub fn get_region_topic_id(region_id: &RegionId) -> Topic {
        Topic::from(<[u8; 32]>::from(region_id.clone()))
    }

    pub async fn subscribe(&self, topic_id: Topic) -> Result<(), PandaSubscriptionError> {
        let node_lock = self.node.lock().await;
        let node = match node_lock.as_ref() {
            Some(node) => node.clone(),
            None => return Err(PandaSubscriptionError::CouldntGetNodeLock()),
        };
        drop(node_lock);

        let (incoming_tx, mut incoming_rx) = mpsc::channel::<IncomingOperation>(32);

        node.subscribe_to_topic(topic_id, incoming_tx).await?;

        let events_tx = self.lores_events_tx.clone();
        tokio::spawn(async move {
            while let Some(incoming) = incoming_rx.recv().await {
                match Self::decode_incoming_to_lores_event(incoming) {
                    Ok(lores_event) => {
                        if events_tx.send(lores_event).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to decode LoResEvent from operation: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn publish_persisted(
        &self,
        topic_id: Topic,
        event_payload: LoResEventPayload,
        current_user: Option<User>,
    ) -> Result<(), PandaPublishError> {
        let node_lock = self.node.lock().await;
        let node = match node_lock.as_ref() {
            Some(node) => node.clone(),
            None => return Err(PandaPublishError::NodeNotStarted),
        };
        drop(node_lock);

        let node_steward_id = match current_user {
            Some(user) if user.is_node_steward() => Some(user.id.clone()),
            _ => None,
        };
        let metadata = LoResEventMetadataV1 { node_steward_id };
        let encoded_payload = encode_lores_event_payload(event_payload, metadata)
            .map_err(|e| PandaPublishError::AppError(format!("Encoding error: {e}")))?;

        node.publish(topic_id, encoded_payload).await?;

        Ok(())
    }

    fn decode_incoming_to_lores_event(
        incoming: IncomingOperation,
    ) -> Result<LoResEvent, anyhow::Error> {
        let lores_header = LoResEventHeader {
            author_node_id: incoming.author.to_hex(),
            region_id: Some(incoming.topic.to_bytes().into()),
            timestamp: incoming.timestamp,
            operation_id: incoming.operation_id,
        };

        decode_lores_event(lores_header, &incoming.bytes)
    }

    pub async fn get_log_counts(&self) -> Result<Vec<LogCount>, anyhow::Error> {
        let node_lock = self.node.lock().await;
        let node = match node_lock.as_ref() {
            Some(node) => node.clone(),
            None => return Err(anyhow::anyhow!("Node not started")),
        };
        drop(node_lock);

        node.get_log_counts()
            .await
            .map_err(|e| anyhow::anyhow!("Error finding log count: {}", e))
    }

    pub async fn get_operation_counts_by_topic(
        &self,
    ) -> Result<Vec<OperationCountByAuthorAndTopic>, anyhow::Error> {
        let node_lock = self.node.lock().await;
        let node = match node_lock.as_ref() {
            Some(node) => node.clone(),
            None => return Err(anyhow::anyhow!("Node not started")),
        };
        drop(node_lock);

        node.get_operation_counts_by_topic()
            .await
            .map_err(|e| anyhow::anyhow!("Error querying operation counts: {}", e))
    }

    /// Validates the node ID and saves it to params. The new peer will be
    /// discovered on the next node restart via the saved config; the high-level
    /// p2panda API does not support adding bootstrap nodes at runtime.
    pub async fn add_bootstrap_node(
        &self,
        bootstrap_node_id_hex: &str,
    ) -> Result<(), anyhow::Error> {
        build_public_key_from_hex(bootstrap_node_id_hex)
            .map_err(|e| anyhow::anyhow!("Invalid bootstrap node ID: {}", e))?;
        Ok(())
    }
}

pub fn build_public_key_from_hex(key_hex: &str) -> Result<PublicKey, anyhow::Error> {
    let key_bytes = hex::decode(key_hex).map_err(|_| anyhow::anyhow!("Invalid hex string"))?;
    let key_byte_array: [u8; PUBLIC_KEY_LEN] = key_bytes
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid public key length"))?;
    PublicKey::from_bytes(&key_byte_array).map_err(|_| anyhow::anyhow!("Invalid public key"))
}
