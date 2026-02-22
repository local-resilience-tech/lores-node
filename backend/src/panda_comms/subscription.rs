use p2panda_core::{Body, Header, Operation};
use p2panda_net::{
    sync::{SyncHandle, SyncHandleError},
    NodeId, TopicId,
};
use p2panda_store::SqliteStore;
use p2panda_stream::IngestExt;
use p2panda_sync::protocols::TopicLogSyncEvent;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

use super::{
    network::{LogSync, LogSyncError},
    operation_store::LOG_ID,
    operations::{LoResMeshExtensions, LoresOperation},
    topic::{LoResNodeTopicMap, LogId},
};

pub type LoResSyncHandleError =
    SyncHandleError<Operation<LoResMeshExtensions>, TopicLogSyncEvent<LoResMeshExtensions>>;

#[derive(Error, Debug)]
pub enum SubscriptionError {
    #[error(transparent)]
    LogSyncError(#[from] LogSyncError),
    #[error(transparent)]
    SyncHandleError(#[from] LoResSyncHandleError),
}

#[derive(Error, Debug)]
pub enum SubscriptionPublishError {
    #[error(transparent)]
    SyncHandleError(#[from] LoResSyncHandleError),
}

pub struct Subscription {
    topic_id: TopicId,
    sync_tx: SyncHandle<Operation<LoResMeshExtensions>, TopicLogSyncEvent<LoResMeshExtensions>>,
}

impl Subscription {
    pub async fn new(
        topic_id: TopicId,
        this_node_id: NodeId,
        log_sync: &LogSync,
        inner_operation_store: SqliteStore<LogId, LoResMeshExtensions>,
        topic_map: &LoResNodeTopicMap,
        operation_tx: &mpsc::Sender<LoresOperation>,
    ) -> Result<Self, SubscriptionError> {
        println!(
            "Starting subscription for topic_id: {:?}",
            hex::encode(topic_id)
        );

        let sync_tx = log_sync.stream(topic_id, true).await?;

        let mut topic_rx = sync_tx.subscribe().await?;

        topic_map.insert(topic_id, this_node_id, LOG_ID).await;

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
                        match validate_and_unpack(operation.as_ref().to_owned(), topic_id) {
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
            .ingest(inner_operation_store, 128)
            .filter_map(|result| match result {
                Ok(operation) => Some(operation),
                Err(err) => {
                    println!("ingesting operation failed: {err}");
                    None
                }
            });

        let operation_tx = operation_tx.clone();

        tokio::task::spawn(async move {
            while let Some(operation) = stream.next().await {
                // Send to operation_tx
                if let Err(e) = operation_tx.send(operation).await {
                    eprintln!("Failed to send operation to channel: {}", e);
                }
            }
        });

        Ok(Subscription { topic_id, sync_tx })
    }

    pub fn has_topic_id(&self, topic_id: &TopicId) -> bool {
        &self.topic_id == topic_id
    }

    pub async fn publish_operation(
        &self,
        operation: Operation<LoResMeshExtensions>,
    ) -> Result<(), SubscriptionPublishError> {
        println!(
            "Publishing operation to LogSync: {:?}",
            operation.hash.to_hex()
        );

        self.sync_tx.publish(operation).await?;

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
    operation: Operation<LoResMeshExtensions>,
    id: TopicId,
) -> Result<OperationWithRawHeader, UnpackError> {
    let Operation::<LoResMeshExtensions> { header, body, .. } = operation;

    let Some(operation_id): Option<TopicId> = header.extension() else {
        return Err(UnpackError::InvalidTopicId);
    };

    if operation_id != id {
        return Err(UnpackError::InvalidTopicId);
    }

    Ok((header.clone(), body, header.to_bytes()))
}
