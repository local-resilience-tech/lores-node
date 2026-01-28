use p2panda_core::{Hash, PrivateKey, PublicKey};
use sqlx::SqlitePool;
use tokio::sync::RwLock;

use super::{network::Network, operation_store::OperationStore, panda_node::PandaNodeError};

#[allow(dead_code)]
pub struct PandaNodeInner {
    network: RwLock<Network>,
}

impl PandaNodeInner {
    pub async fn new(
        network_id: Hash,
        private_key: PrivateKey,
        bootstrap_node_id: Option<PublicKey>,
        operations_pool: &SqlitePool,
    ) -> Result<Self, PandaNodeError> {
        println!("Initializing PandaNodeInner...");

        let operation_store = OperationStore::new(operations_pool.clone());

        let network =
            Network::new(network_id, private_key, bootstrap_node_id, &operation_store).await?;

        Ok(PandaNodeInner {
            network: RwLock::new(network),
        })
    }
}
