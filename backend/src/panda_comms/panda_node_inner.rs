use p2panda_core::{cbor::EncodeError, Hash, PrivateKey, PublicKey};
use sqlx::SqlitePool;
use thiserror::Error;
use tokio::sync::RwLock;

use crate::api::auth_api::auth_backend::User;

use super::{
    event_encoding::encode_lores_event_payload,
    lores_events::{LoResEventMetadataV1, LoResEventPayload},
    network::Network,
    operation_store::{CreationError, OperationStore},
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
        })
    }

    pub async fn publish_persisted(
        &self,
        event_payload: LoResEventPayload,
        current_user: Option<User>,
    ) -> Result<(), PandaPublishError> {
        let node_steward_id = match current_user {
            Some(user) if user.is_node_steward() => Some(user.id.clone()),
            _ => None,
        };
        let metadata = LoResEventMetadataV1 { node_steward_id };

        let encoded_payload = encode_lores_event_payload(event_payload, metadata)?;

        let operation = self
            .operation_store
            .create_operation(&self.private_key, Some(&encoded_payload))
            .await?;

        // Publish the operation to the network
        let network = self.network.write().await;
        network
            .publish_operation(operation)
            .await
            .map_err(|e| PandaPublishError::SyncError(e.to_string()))?;

        Ok(())
    }
}
