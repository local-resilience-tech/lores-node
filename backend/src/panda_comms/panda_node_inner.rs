use p2panda_core::{cbor::EncodeError, Body, Hash, Header, PrivateKey, PublicKey};
use p2panda_net::TopicId;
use p2panda_stream::IngestExt;
use p2panda_sync::protocols::TopicLogSyncEvent;
use sqlx::SqlitePool;
use thiserror::Error;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

use super::{
    network::{Network, NetworkError},
    operation_store::{CreationError, OperationStore},
    operations::{LoResMeshExtensions, LoresOperation},
    panda_node::PandaNodeError,
    subscription::Subscription,
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
    #[error("Operation error: {0}")]
    OperationError(String),
    #[error("App error: {0}")]
    AppError(String),
}

#[allow(dead_code)]
pub struct PandaNodeInner {
    network: RwLock<Network>,
    pub operation_store: OperationStore,
    private_key: PrivateKey,
    subscriptions: RwLock<Vec<Subscription>>,
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
            subscriptions: RwLock::new(Vec::new()),
        })
    }

    pub async fn publish_persisted(
        &self,
        topic_id: TopicId,
        encoded_payload: &Vec<u8>,
    ) -> Result<LoresOperation, PandaPublishError> {
        let operation = self
            .operation_store
            .create_operation(topic_id, &self.private_key, Some(encoded_payload))
            .await?;

        // Publish the operation to the network
        let network = self.network.write().await;
        network
            .publish_operation(operation.clone())
            .await
            .map_err(|e| PandaPublishError::SyncError(e.to_string()))?;

        Ok(operation)
    }

    pub async fn subscribe_to_topic(
        &self,
        topic_id: TopicId,
        operation_tx: mpsc::Sender<LoresOperation>,
    ) -> Result<(), PandaNodeError> {
        let network = self.network.write().await;

        let log_sync = network.get_log_sync();

        let subscription = Subscription::new(
            topic_id,
            &log_sync,
            self.operation_store.clone_inner(),
            &operation_tx,
        )
        .await?;
        self.subscriptions.write().await.push(subscription);

        Ok(())
    }
}
