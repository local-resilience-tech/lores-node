use std::pin::Pin;
use std::sync::Arc;

use futures::StreamExt;
use lores_p2panda_client::{PandaClient, PandaError, PublishResult};
use tokio::sync::Mutex;

use crate::{
    stores::{OperationStore, OperationStream, RawOperationEvent, StoreError, StorePublishResult},
    LoResNodeId, LoResOperationId,
};

impl From<PandaError> for StoreError {
    fn from(e: PandaError) -> Self {
        match e {
            PandaError::RegionNotBound(msg) => StoreError::RegionNotBound(msg),
            PandaError::Rpc(s) => StoreError::Other(s.to_string()),
        }
    }
}

/// [`OperationStore`] implementation that forwards operations to a lores-node
/// instance via gRPC using [`PandaClient`].
pub(crate) struct GrpcOperationStore {
    client: Arc<Mutex<PandaClient>>,
    app_id: String,
    instance_id: String,
}

impl GrpcOperationStore {
    pub(crate) fn new(client: Arc<Mutex<PandaClient>>, app_id: impl Into<String>, instance_id: impl Into<String>) -> Self {
        Self {
            client,
            app_id: app_id.into(),
            instance_id: instance_id.into(),
        }
    }
}

impl OperationStore for GrpcOperationStore {
    fn publish(
        &mut self,
        payload: Vec<u8>,
        idempotency_key: Option<String>,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<StorePublishResult, StoreError>> + Send + '_>> {
        Box::pin(async move {
            let PublishResult { operation_id, node_id } = self
                .client
                .lock()
                .await
                .publish(&self.app_id, &self.instance_id, payload, idempotency_key.map(|k| k.into_bytes()))
                .await
                .map_err(StoreError::from)?;
            Ok(StorePublishResult {
                operation_id: operation_id.into_non_empty().map(LoResOperationId),
                node_id: node_id.into_non_empty().map(LoResNodeId),
            })
        })
    }

    fn subscribe(&mut self) -> Pin<Box<dyn std::future::Future<Output = Result<OperationStream, StoreError>> + Send + '_>> {
        Box::pin(async move {
            let response = self
                .client
                .lock()
                .await
                .subscribe(&self.app_id, &self.instance_id)
                .await
                .map_err(StoreError::from)?;

            let stream: OperationStream = Box::pin(response.into_inner().map(|item| {
                item.map(|event| RawOperationEvent {
                    payload: event.payload,
                    author: Some(event.author),
                    operation_id: Some(event.operation_id),
                    timestamp: Some(event.timestamp),
                })
                .map_err(|s| StoreError::Other(s.to_string()))
            }));

            Ok(stream)
        })
    }
}
