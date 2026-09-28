use std::pin::Pin;
use std::sync::Arc;

use futures::StreamExt;
use lores_p2panda_client::proto::subscribe_event::Event as SubscribeEventKind;
use lores_p2panda_client::{PandaClient, PandaError, PublishResult, SubscriptionFrom};
use tokio::sync::Mutex;

use crate::{
    NodeId, OperationId,
    transports::{OperationStream, OperationTransport, RawEvent, RawOperationEvent, TransportError, TransportPublishResult},
};

impl From<PandaError> for TransportError {
    fn from(e: PandaError) -> Self {
        match e {
            PandaError::RegionNotBound(msg) => TransportError::RegionNotBound(msg),
            PandaError::Rpc(s) => TransportError::Other(s.to_string()),
        }
    }
}

/// [`OperationTransport`] implementation that forwards operations to a lores-node
/// instance via gRPC using [`PandaClient`].
pub(crate) struct GrpcTransport {
    client: Arc<Mutex<PandaClient>>,
    app_id: String,
    instance_id: String,
}

impl GrpcTransport {
    pub(crate) fn new(client: Arc<Mutex<PandaClient>>, app_id: impl Into<String>, instance_id: impl Into<String>) -> Self {
        Self {
            client,
            app_id: app_id.into(),
            instance_id: instance_id.into(),
        }
    }
}

impl OperationTransport for GrpcTransport {
    fn publish(
        &mut self,
        payload: Vec<u8>,
        idempotency_key: Option<String>,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<TransportPublishResult, TransportError>> + Send + '_>> {
        Box::pin(async move {
            let PublishResult { operation_id, node_id } = self
                .client
                .lock()
                .await
                .publish(&self.app_id, &self.instance_id, payload, idempotency_key.map(|k| k.into_bytes()))
                .await
                .map_err(TransportError::from)?;
            Ok(TransportPublishResult {
                operation_id: operation_id.into_non_empty().map(OperationId),
                node_id: node_id.into_non_empty().map(NodeId),
            })
        })
    }

    fn subscribe(
        &mut self,
        start_from: SubscriptionFrom,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<OperationStream, TransportError>> + Send + '_>> {
        Box::pin(async move {
            let response = self
                .client
                .lock()
                .await
                .subscribe(&self.app_id, &self.instance_id, start_from)
                .await
                .map_err(TransportError::from)?;

            let stream: OperationStream = Box::pin(response.into_inner().filter_map(|item| async move {
                match item {
                    Ok(event) => match event.event {
                        Some(SubscribeEventKind::Operation(op)) => Some(Ok(RawEvent::Operation(RawOperationEvent {
                            payload: op.payload,
                            author: Some(op.author),
                            operation_id: Some(op.operation_id),
                            timestamp: Some(op.timestamp),
                        }))),
                        Some(SubscribeEventKind::ReplayStarted(rs)) => Some(Ok(RawEvent::ReplayStarted {
                            total_operations: rs.total_operations,
                        })),
                        Some(SubscribeEventKind::ReplayEnded(_)) => Some(Ok(RawEvent::ReplayEnded)),
                        None => None,
                    },
                    Err(s) => Some(Err(TransportError::Other(s.to_string()))),
                }
            }));

            Ok(stream)
        })
    }
}
