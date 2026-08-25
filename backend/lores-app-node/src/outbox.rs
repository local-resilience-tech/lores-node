use std::pin::Pin;

use crate::grpc::GrpcOperationStore;
use crate::local::LocalOperationStore;
use crate::store::{OperationStore, OperationStream, StoreError};

/// [`OperationStore`] decorator that combines a [`LocalOperationStore`] and a
/// [`GrpcOperationStore`].
///
/// On publish:
/// 1. The payload is inserted into the local store, which assigns a stable row
///    id used as the idempotency key.
/// 2. The payload is forwarded to lores-node via gRPC with that key.
/// 3. On successful delivery the local entry is deleted; on failure it remains
///    for a future drain attempt.
pub(crate) struct OutboxStore {
    local: LocalOperationStore,
    remote: GrpcOperationStore,
}

impl OutboxStore {
    pub(crate) fn new(local: LocalOperationStore, remote: GrpcOperationStore) -> Self {
        Self { local, remote }
    }
}

impl OperationStore for OutboxStore {
    fn publish(
        &mut self,
        payload: Vec<u8>,
        _idempotency_key: Option<String>,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), StoreError>> + Send + '_>> {
        Box::pin(async move {
            // 1. Persist locally — this is our source of truth until gRPC acks.
            let id = self
                .local
                .insert(payload.clone())
                .await
                .map_err(|e| StoreError::Other(e.to_string()))?;

            let key = Some(id.to_string());

            // 2. Attempt gRPC delivery.
            match self.remote.publish(payload, key).await {
                Ok(()) => {
                    // 3. Confirmed — remove from local store.
                    if let Err(e) = self.local.delete(id).await {
                        tracing::warn!("Delivered op {id} but failed to delete from local store: {e}");
                    }
                    Ok(())
                }
                Err(e) => {
                    tracing::warn!("gRPC delivery failed for op {id}: {e}");
                    // Leave in local store for a future drain attempt.
                    Ok(())
                }
            }
        })
    }

    fn subscribe(
        &mut self,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<OperationStream, StoreError>> + Send + '_>> {
        self.remote.subscribe()
    }

    fn replay(
        &mut self,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<OperationStream, StoreError>> + Send + '_>> {
        self.local.replay()
    }
}
