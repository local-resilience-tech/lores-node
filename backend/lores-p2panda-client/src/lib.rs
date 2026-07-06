use tonic::transport::Channel;

pub mod proto {
    tonic::include_proto!("lores.panda.v1");
}

use proto::{OperationEvent, PublishRequest, PublishResponse, SubscribeRequest, panda_client::PandaClient as TonicPandaClient};
use tonic::{Response, Status, Streaming};

/// Client for the lores-p2panda-server gRPC API.
pub struct PandaClient {
    inner: TonicPandaClient<Channel>,
}

impl PandaClient {
    /// Connect to a lores-p2panda-server at the given endpoint URI.
    ///
    /// # Example
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// let client = lores_p2panda_client::PandaClient::connect("http://[::1]:50051").await.unwrap();
    /// # });
    /// ```
    pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: TryInto<tonic::transport::Endpoint>,
        D::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let inner = TonicPandaClient::connect(dst).await?;
        Ok(Self { inner })
    }

    /// Create a client with a lazy channel — no connection is made until the
    /// first RPC call, so the process starts cleanly even if the gRPC server
    /// is not yet available.
    pub fn connect_lazy<D>(dst: D) -> Result<Self, tonic::transport::Error>
    where
        D: TryInto<tonic::transport::Endpoint>,
        D::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let endpoint = tonic::transport::Endpoint::new(dst)?;
        let inner = TonicPandaClient::new(endpoint.connect_lazy());
        Ok(Self { inner })
    }

    /// Publish an operation to a region+namespace topic.
    ///
    /// Returns only after the operation has been persisted by the remote
    /// p2panda node, guaranteeing eventual propagation to peers.
    ///
    /// If `idempotency_key` is `Some`, the server will deduplicate within its
    /// retention window: retrying with the same key returns success without
    /// re-inserting the operation.
    pub async fn publish(
        &mut self,
        app_id: impl Into<String>,
        instance_id: impl Into<String>,
        payload: impl Into<Vec<u8>>,
        idempotency_key: Option<Vec<u8>>,
    ) -> Result<Response<PublishResponse>, Status> {
        let request = PublishRequest {
            app_id: app_id.into(),
            instance_id: instance_id.into(),
            payload: payload.into(),
            idempotency_key: idempotency_key.unwrap_or_default(),
        };
        self.inner.publish(request).await
    }

    /// Subscribe to a region+namespace topic and receive a stream of
    /// [`OperationEvent`]s.
    ///
    /// HTTP/2 flow control provides natural backpressure.
    pub async fn subscribe(
        &mut self,
        app_id: impl Into<String>,
        instance_id: impl Into<String>,
    ) -> Result<Response<Streaming<OperationEvent>>, Status> {
        let request = SubscribeRequest {
            app_id: app_id.into(),
            instance_id: instance_id.into(),
        };
        self.inner.subscribe(request).await
    }
}
