use p2panda_core::{cbor::EncodeError, Hash, PrivateKey, PublicKey};
use p2panda_net::{sync::SyncSubscription, TopicId};
use p2panda_sync::protocols::TopicLogSyncEvent;
use sqlx::SqlitePool;
use thiserror::Error;
use tokio::sync::RwLock;

use super::{
    network::{Network, NetworkError},
    operation_store::{CreationError, OperationStore},
    operations::LoResMeshExtensions,
    panda_node::PandaNodeError,
};

#[derive(Error, Debug)]
pub enum PandaPublishError {
    #[error(transparent)]
    CreationError(#[from] CreationError),
    #[error(transparent)]
    EncodeError(#[from] EncodeError),
    #[error("Node not started")]
    NodeNotStarted,
    #[error("Sync error: {0}")]
    SyncError(String),
}

#[allow(dead_code)]
pub struct PandaNodeInner {
    network: RwLock<Network>,
    operation_store: OperationStore,
    private_key: PrivateKey,
}

impl PandaNodeInner {
    pub async fn new(
        network_id: Hash,
        private_key: PrivateKey,
        admin_topic_id: TopicId,
        bootstrap_node_id: Option<PublicKey>,
        operations_pool: &SqlitePool,
    ) -> Result<Self, PandaNodeError> {
        println!("Initializing PandaNodeInner...");

        let operation_store = OperationStore::new(operations_pool.clone());

        let network = Network::new(
            network_id,
            private_key.clone(),
            admin_topic_id,
            bootstrap_node_id,
            &operation_store,
        )
        .await?;

        Ok(PandaNodeInner {
            network: RwLock::new(network),
            operation_store,
            private_key,
        })
    }

    pub async fn publish_persisted(
        &self,
        encoded_payload: &Vec<u8>,
    ) -> Result<(), PandaPublishError> {
        let operation = self
            .operation_store
            .create_operation(&self.private_key, Some(encoded_payload))
            .await?;

        // Publish the operation to the network
        let network = self.network.write().await;
        network
            .publish_operation(operation)
            .await
            .map_err(|e| PandaPublishError::SyncError(e.to_string()))?;

        Ok(())
    }

    pub async fn subscribe_to_admin_topic(
        &self,
    ) -> Result<SyncSubscription<TopicLogSyncEvent<LoResMeshExtensions>>, PandaNodeError> {
        let network = self.network.write().await;

        let sync_tx = network.get_sync_handle();
        let sync_rx = sync_tx.subscribe().await.map_err(|e| {
            NetworkError::SyncHandleError(format!("Failed to subscribe to log sync: {}", e))
        })?;

        Ok(sync_rx)
    }
}
