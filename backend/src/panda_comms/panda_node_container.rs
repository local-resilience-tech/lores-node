use p2panda_core::{identity::PUBLIC_KEY_LEN, Hash, PrivateKey, PublicKey};
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::panda_node::{PandaNode, PandaNodeError};

#[derive(Default, Clone)]
pub struct NodeParams {
    pub private_key: Option<PrivateKey>,
    pub network_name: Option<String>,
    pub bootstrap_node_id: Option<PublicKey>,
}

#[derive(Clone)]
pub struct PandaNodeContainer {
    params: Arc<Mutex<NodeParams>>,
    node: Arc<Mutex<Option<PandaNode>>>,
}

impl PandaNodeContainer {
    pub fn new() -> Self {
        let params = Arc::new(Mutex::new(NodeParams::default()));

        PandaNodeContainer {
            params,
            node: Arc::new(Mutex::new(None)),
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

    pub async fn set_bootstrap_node_id(&self, bootstrap_node_id: Option<PublicKey>) {
        let mut params_lock = self.params.lock().await;
        params_lock.bootstrap_node_id = bootstrap_node_id;
    }

    pub async fn start(&self, operations_pool: &SqlitePool) -> Result<(), PandaNodeError> {
        println!("Starting client");

        let params = self.get_params().await;

        let private_key: Option<PrivateKey> = params.private_key;
        let network_name: Option<String> = params.network_name;
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
            .await
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
            bootstrap_node_id: boostrap_node_id,
        };

        let panda_node = PandaNode::new(&required_params).await?;

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

    // async fn start_for(
    //     &self,
    //     private_key: PrivateKey,
    //     network_name: String,
    //     boostrap_node_id: Option<PublicKey>,
    //     store_pool: &SqlitePool,
    // ) -> Result<()> {
    //     let relay_url: RelayUrl = RELAY_URL.parse().unwrap();
    //     let temp_blobs_root_dir = tempfile::tempdir().expect("temp dir");

    //     let store = SqliteStore::<LogId, NodeExtensions>::new(store_pool.clone());

    //     let topic_map = TopicMap::new();

    //     println!(
    //         "Starting node. Network name: {}, Bootstrap ID: {:?}",
    //         network_name,
    //         boostrap_node_id.map(|key| key.to_string())
    //     );

    //     let (node, stream_rx, network_events_rx) = Node::new(
    //         network_name,
    //         private_key.clone(),
    //         boostrap_node_id,
    //         Some(relay_url),
    //         store,
    //         temp_blobs_root_dir.keep(),
    //         topic_map.clone(),
    //     )
    //     .await?;

    //     let mut node_api = NodeApi::new(node, topic_map);

    //     let public_key = private_key.public_key();

    //     node_api
    //         .add_topic_log(&public_key, TOPIC_NAME, LOG_ID)
    //         .await?;

    //     // subscribe to main topic
    //     let result = node_api.subscribe_persisted(TOPIC_NAME).await;
    //     match result {
    //         Ok(_) => println!("Subscribed to topic: {}", TOPIC_NAME),
    //         Err(err) => {
    //             eprintln!("Failed to subscribe to topic {}: {}", TOPIC_NAME, err);
    //         }
    //     }

    //     // put the node in the container
    //     self.set_node_api(Some(node_api)).await;

    //     self.listen_for_messages(stream_rx, network_events_rx);

    //     Ok(())
    // }
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
