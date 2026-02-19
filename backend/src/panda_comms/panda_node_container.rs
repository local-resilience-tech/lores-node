use futures_util::StreamExt;
use p2panda_core::{identity::PUBLIC_KEY_LEN, Hash, Operation, PrivateKey, PublicKey};
use p2panda_net::TopicId;
use sqlx::SqlitePool;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::ReceiverStream;

use crate::{api::auth_api::auth_backend::User, panda_comms::event_encoding::decode_lores_event};

use super::{
    event_encoding::encode_lores_event_payload,
    lores_events::{LoResEvent, LoResEventHeader, LoResEventMetadataV1, LoResEventPayload},
    operations::{LoResMeshExtensions, LoresOperation},
    panda_node::{PandaNode, PandaNodeError},
    panda_node_inner::PandaPublishError,
};

pub const NODE_ADMIN_TOPIC_ID: TopicId = [0u8; 32];

#[derive(Default, Clone)]
pub struct NodeParams {
    pub private_key: Option<PrivateKey>,
    pub region_name: Option<String>,
    pub bootstrap_node_id: Option<PublicKey>,
}

#[derive(Debug, Error)]
pub enum PandaNodeContainerError {
    #[error(transparent)]
    PandaNodeError(#[from] PandaNodeError),
    #[error("Panda subscribe error")]
    PandaSubscribeError(),
}

#[derive(Clone)]
pub struct PandaNodeContainer {
    params: Arc<Mutex<NodeParams>>,
    node: Arc<Mutex<Option<PandaNode>>>,
    #[allow(dead_code)]
    events_tx: mpsc::Sender<LoResEvent>,
}

impl PandaNodeContainer {
    pub fn new(events_tx: mpsc::Sender<LoResEvent>) -> Self {
        let params = Arc::new(Mutex::new(NodeParams::default()));

        PandaNodeContainer {
            params,
            node: Arc::new(Mutex::new(None)),
            events_tx,
        }
    }

    pub async fn get_params(&self) -> NodeParams {
        let params_lock = self.params.lock().await;
        params_lock.clone()
    }

    pub async fn set_region_name(&self, region_name: String) {
        let mut params_lock = self.params.lock().await;
        params_lock.region_name = Some(region_name);
    }

    pub async fn set_private_key(&self, private_key: PrivateKey) {
        let mut params_lock = self.params.lock().await;
        params_lock.private_key = Some(private_key);
    }

    pub async fn set_bootstrap_node_id(&self, bootstrap_node_id: Option<PublicKey>) {
        let mut params_lock = self.params.lock().await;
        params_lock.bootstrap_node_id = bootstrap_node_id;
    }

    pub async fn start(&self, operations_pool: &SqlitePool) -> Result<(), PandaNodeContainerError> {
        println!("Starting client");

        let params = self.get_params().await;

        let private_key: Option<PrivateKey> = params.private_key;
        let network_name: Option<String> = params.region_name;
        let boostrap_node_id: Option<PublicKey> = params.bootstrap_node_id;

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

        self.start_for(private_key, network_name, boostrap_node_id, operations_pool)
            .await?;

        self.start_subscriptions().await?;

        Ok(())
    }

    async fn start_for(
        &self,
        private_key: PrivateKey,
        network_name: String,
        boostrap_node_id: Option<PublicKey>,
        operations_pool: &SqlitePool,
    ) -> Result<(), PandaNodeError> {
        let required_params = super::panda_node::RequiredNodeParams {
            private_key,
            network_id: Hash::new(network_name.as_bytes()),
            admin_topic_id: NODE_ADMIN_TOPIC_ID,
            bootstrap_node_id: boostrap_node_id,
        };

        let panda_node = PandaNode::new(&required_params, operations_pool).await?;

        {
            let mut node_lock = self.node.lock().await;
            *node_lock = Some(panda_node);
        }

        println!(
            "P2Panda: Node started. Network name: {}, Bootstrap ID: {:?}",
            network_name,
            boostrap_node_id.map(|key| key.to_string())
        );

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

    async fn start_subscriptions(&self) -> Result<(), PandaNodeContainerError> {
        let node_lock = self.node.lock().await;

        let node = match node_lock.as_ref() {
            Some(node) => node,
            None => return Err(PandaNodeContainerError::PandaSubscribeError()),
        };

        let (operation_tx, operation_rx): (
            mpsc::Sender<LoresOperation>,
            mpsc::Receiver<LoresOperation>,
        ) = mpsc::channel(32);

        // Received messages on operation_rx
        {
            let events_tx = self.events_tx.clone();

            tokio::task::spawn(async move {
                println!("  P2Panda Network initialized, starting sync stream...");
                let mut operation_rx = ReceiverStream::new(operation_rx);
                while let Some(operation) = operation_rx.next().await {
                    match Self::decode_operation_to_lores_event(&operation) {
                        Ok(lores_event) => {
                            if let Err(e) = events_tx.send(lores_event).await {
                                eprintln!("  Failed to send LoResEvent: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("  Failed to decode LoResEvent from operation: {}", e);
                            continue;
                        }
                    }
                }
                println!("  Sync stream read loop ended.");
            });
        }

        node.inner.subscribe_to_admin_topic(operation_tx).await?;

        Ok(())
    }

    pub async fn publish_persisted(
        &self,
        event_payload: LoResEventPayload,
        current_user: Option<User>,
    ) -> Result<(), PandaPublishError> {
        let node_lock = self.node.lock().await;
        let node = match node_lock.as_ref() {
            Some(node) => node,
            None => return Err(PandaPublishError::NodeNotStarted),
        };

        let node_steward_id = match current_user {
            Some(user) if user.is_node_steward() => Some(user.id.clone()),
            _ => None,
        };
        let metadata = LoResEventMetadataV1 { node_steward_id };
        let encoded_payload = encode_lores_event_payload(event_payload, metadata)?;

        let operation = node.inner.publish_persisted(&encoded_payload).await?;

        // Since this came from this node, it wont be received via subscription, so we
        // need to handle it ourselves.
        let lores_event = Self::decode_operation_to_lores_event(&operation).map_err(|e| {
            PandaPublishError::OperationError(format!(
                "Failed to decode LoResEvent from published operation: {}",
                e
            ))
        })?;
        self.events_tx.send(lores_event).await.map_err(|e| {
            PandaPublishError::AppError(format!(
                "Failed to send LoResEvent back to the application: {}",
                e
            ))
        })?;

        Ok(())
    }

    fn decode_operation_to_lores_event(
        operation: &Operation<LoResMeshExtensions>,
    ) -> Result<LoResEvent, anyhow::Error> {
        let operation_header = &operation.header;
        let lores_header = LoResEventHeader {
            author_node_id: operation_header.public_key.to_hex(),
            timestamp: operation_header.timestamp,
            operation_id: operation_header.hash(),
        };

        let body_bytes: Vec<u8> = match &operation.body {
            Some(body) => body.to_bytes(),
            None => {
                return Err(anyhow::anyhow!(
                    "Operation body is None, cannot decode LoResEvent"
                ))
            }
        };

        decode_lores_event(lores_header, &body_bytes)
    }
}

// // TODO: This should be in p2panda-core, submit a PR
pub fn build_public_key_from_hex(key_hex: String) -> Option<PublicKey> {
    let key_bytes = hex::decode(key_hex).ok()?;
    let key_byte_array: [u8; PUBLIC_KEY_LEN] = key_bytes.try_into().ok()?;
    let result = PublicKey::from_bytes(&key_byte_array);

    match result {
        Ok(key) => Some(key),
        Err(_) => None,
    }
}
