use p2panda_core::{cbor::EncodeError, Hash, PrivateKey, PublicKey};
use p2panda_net::TopicId;

use sqlx::SqlitePool;
use thiserror::Error;
use tokio::sync::{mpsc, RwLock};

use super::{
    network::Network,
    operation_store::{CreationError, OperationStore},
    operations::LoresOperation,
    panda_node::PandaNodeError,
    subscription::{Subscription, SubscriptionPublishError},
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
    OperationError(String),
    #[error("App error: {0}")]
    AppError(String),
    #[error(transparent)]
    SubscriptionPublishError(#[from] SubscriptionPublishError),
    #[error("No subscription found for topic_id: {0:?}")]
    NoSubscriptionFound(TopicId),
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
        bootstrap_node_id: Option<PublicKey>,
        operations_pool: &SqlitePool,
    ) -> Result<Self, PandaNodeError> {
        println!("Initializing PandaNodeInner...");

        let operation_store = OperationStore::new(operations_pool.clone());

        let network = Network::new(
            network_id,
            private_key.clone(),
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

        let subscriptions = self.subscriptions.blocking_read();
        let subscription = subscriptions.iter().find(|s| s.has_topic_id(&topic_id));

        if let Some(subscription) = subscription {
            subscription.publish_operation(operation.clone()).await?;
        } else {
            println!("No subscription found for topic_id: {:?}", topic_id);
            return Err(PandaPublishError::NoSubscriptionFound(topic_id));
        }

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
            self.private_key.public_key(),
            &log_sync,
            self.operation_store.clone_inner(),
            &network.topic_map,
            &operation_tx,
        )
        .await?;
        self.subscriptions.write().await.push(subscription);

        Ok(())
    }
}
