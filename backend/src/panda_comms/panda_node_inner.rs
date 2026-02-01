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
    panda_node_container::NODE_ADMIN_TOPIC_ID,
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
    pub operation_store: OperationStore,
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
        operation_tx: mpsc::Sender<LoresOperation>,
    ) -> Result<(), PandaNodeError> {
        let network = self.network.write().await;

        let sync_tx = network.get_sync_handle();
        let mut topic_rx = sync_tx.subscribe().await.map_err(|e| {
            NetworkError::SyncHandleError(format!("Failed to subscribe to log sync: {}", e))
        })?;

        let (persistent_tx, persistent_rx) =
            mpsc::channel::<(Header<LoResMeshExtensions>, Option<Body>, Vec<u8>)>(128);

        let stream = ReceiverStream::new(persistent_rx);

        tokio::task::spawn(async move {
            while let Some(event) = topic_rx.next().await {
                let event = match event {
                    Ok(event) => event,
                    Err(error) => {
                        eprintln!("Error while receiving sync message: {error}");
                        continue;
                    }
                };
                match event.event() {
                    TopicLogSyncEvent::Operation(operation) => {
                        match validate_and_unpack(
                            operation.as_ref().to_owned(),
                            NODE_ADMIN_TOPIC_ID,
                        ) {
                            Ok(data) => {
                                persistent_tx.send(data).await.unwrap();
                            }
                            Err(err) => {
                                eprintln!("Failed to unpack operation: {err}");
                            }
                        }
                    }
                    _ => {
                        // TODO: Handle sync events
                    }
                }
            }
        });

        let mut stream = stream
            // NOTE(adz): The persisting part should happen later, we want to check the payload on
            // application layer first. In general "ingest" does too much at once and is
            // inflexible. Related issue: https://github.com/p2panda/p2panda/issues/696
            .ingest(self.operation_store.clone_inner(), 128)
            .filter_map(|result| match result {
                Ok(operation) => Some(operation),
                Err(err) => {
                    println!("ingesting operation failed: {err}");
                    None
                }
            });

        tokio::task::spawn(async move {
            while let Some(operation) = stream.next().await {
                // Send to operation_tx
                if let Err(e) = operation_tx.send(operation).await {
                    eprintln!("Failed to send operation to channel: {}", e);
                }

                // // Forward the payload up to the app.
                // if let Some(body) = operation.body {
                //     println!(
                //         "ready to forward operation from sync stream: {:?}",
                //         operation.header
                //     );
                //     // subscribable_topic_clone
                //     //     .bytes_received(operation.header.public_key, body.to_bytes());
                // }
            }
        });

        Ok(())
    }
}

type OperationWithRawHeader = (Header<LoResMeshExtensions>, Option<Body>, Vec<u8>);

#[derive(Debug, thiserror::Error)]
pub enum UnpackError {
    #[error(transparent)]
    Cbor(#[from] p2panda_core::cbor::DecodeError),
    #[error("Operation with invalid topic id")]
    InvalidTopicId,
}

fn validate_and_unpack(
    operation: p2panda_core::Operation<LoResMeshExtensions>,
    id: TopicId,
) -> Result<OperationWithRawHeader, UnpackError> {
    let p2panda_core::Operation::<LoResMeshExtensions> { header, body, .. } = operation;

    let Some(operation_id): Option<TopicId> = header.extension() else {
        return Err(UnpackError::InvalidTopicId);
    };

    if operation_id != id {
        return Err(UnpackError::InvalidTopicId);
    }

    Ok((header.clone(), body, header.to_bytes()))
}
