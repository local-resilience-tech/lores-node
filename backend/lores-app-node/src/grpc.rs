use std::pin::Pin;

use futures::StreamExt;
use lores_p2panda_client::{PandaClient, PandaError};

use crate::store::{OperationStore, OperationStream, RawOperationEvent, StoreError};

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
    client: PandaClient,
    app_id: String,
    instance_id: String,
}

impl GrpcOperationStore {
    pub(crate) fn connect_lazy(
        grpc_addr: String,
        app_id: impl Into<String>,
        instance_id: impl Into<String>,
    ) -> Result<Self, tonic::transport::Error> {
        let client = PandaClient::connect_lazy(grpc_addr)?;
        Ok(Self {
            client,
            app_id: app_id.into(),
            instance_id: instance_id.into(),
        })
    }
}

impl OperationStore for GrpcOperationStore {
    fn publish(
        &mut self,
        payload: Vec<u8>,
        idempotency_key: Option<String>,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<(), StoreError>> + Send + '_>> {
        Box::pin(async move {
            self.client
                .publish(
                    &self.app_id,
                    &self.instance_id,
                    payload,
                    idempotency_key.map(|k| k.into_bytes()),
                )
                .await
                .map(|_| ())
                .map_err(StoreError::from)
        })
    }

    fn subscribe(
        &mut self,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<OperationStream, StoreError>> + Send + '_>> {
        Box::pin(async move {
            let response = self
                .client
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
