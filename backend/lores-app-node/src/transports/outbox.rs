use std::pin::Pin;

use lores_p2panda_client::SubscriptionFrom;

use crate::transports::grpc::GrpcTransport;
use crate::transports::local::LocalTransport;
use crate::transports::{OperationStream, OperationTransport, TransportError, TransportPublishResult};

/// [`OperationTransport`] decorator that combines a [`LocalTransport`] and a
/// [`GrpcTransport`].
///
/// On publish:
/// 1. The payload is inserted into the local transport, which assigns a stable row
///    id used as the idempotency key.
/// 2. The payload is forwarded to lores-node via gRPC with that key.
/// 3. On successful delivery the local entry is deleted; on failure it remains
///    for a future drain attempt.
pub(crate) struct OutboxTransport {
    local: LocalTransport,
    remote: GrpcTransport,
}

impl OutboxTransport {
    pub(crate) fn new(local: LocalTransport, remote: GrpcTransport) -> Self {
        Self { local, remote }
    }
}

impl OperationTransport for OutboxTransport {
    fn publish(
        &mut self,
        payload: Vec<u8>,
        idempotency_key: Option<String>,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<TransportPublishResult, TransportError>> + Send + '_>> {
        Box::pin(async move {
            // 1. Persist locally — this is our source of truth until gRPC acks.
            let id = self
                .local
                .insert(payload.clone())
                .await
                .map_err(|e| TransportError::Other(e.to_string()))?;

            // 2. Attempt gRPC delivery.
            match self.remote.publish(payload, idempotency_key).await {
                Ok(result) => {
                    // 3. Confirmed — remove from local transport.
                    if let Err(e) = self.local.delete(id).await {
                        tracing::warn!("Delivered op {id} but failed to delete from local transport: {e}");
                    }
                    Ok(result)
                }
                Err(e) => {
                    tracing::warn!("gRPC delivery failed for op {id}: {e}");
                    // Leave in local transport for a future drain attempt.
                    Ok(TransportPublishResult {
                        operation_id: None,
                        node_id: None,
                    })
                }
            }
        })
    }

    fn subscribe(
        &mut self,
        start_from: SubscriptionFrom,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<OperationStream, TransportError>> + Send + '_>> {
        self.remote.subscribe(start_from)
    }
}
