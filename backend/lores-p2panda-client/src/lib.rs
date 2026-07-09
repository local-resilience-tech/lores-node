use tonic::transport::Channel;

pub mod proto {
    tonic::include_proto!("lores.panda.v2");
}

use proto::{
    OperationEvent, PublishRequest, PublishResponse, SubscribeRequest,
    panda_client::PandaClient as TonicPandaClient,
};
use tonic::{Code, Response, Status, Streaming};

/// Errors returned by [`PandaClient`] methods.
#[derive(Debug)]
pub enum PandaError {
    /// No region has been bound to the given app/instance on the server.
    /// Use your lores-node installation to bind the app to a region.
    /// The inner string is the human-readable message from the server.
    RegionNotBound(String),
    /// Any other gRPC-level error.
    Rpc(Status),
}

impl std::fmt::Display for PandaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PandaError::RegionNotBound(msg) => write!(f, "{msg}"),
            PandaError::Rpc(s) => write!(f, "RPC error: {s}"),
        }
    }
}

impl std::error::Error for PandaError {}

impl From<Status> for PandaError {
    fn from(s: Status) -> Self {
        if s.code() == Code::NotFound {
            PandaError::RegionNotBound(s.message().to_string())
        } else {
            PandaError::Rpc(s)
        }
    }
}

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
    ) -> Result<Response<PublishResponse>, PandaError> {
        let request = PublishRequest {
            app_id: app_id.into(),
            instance_id: instance_id.into(),
            payload: payload.into(),
            idempotency_key: idempotency_key.unwrap_or_default(),
        };
        self.inner.publish(request).await.map_err(PandaError::from)
    }

    /// Subscribe to a region+namespace topic and receive a stream of
    /// [`OperationEvent`]s.
    ///
    /// HTTP/2 flow control provides natural backpressure.
    pub async fn subscribe(
        &mut self,
        app_id: impl Into<String>,
        instance_id: impl Into<String>,
    ) -> Result<Response<Streaming<OperationEvent>>, PandaError> {
        let request = SubscribeRequest {
            app_id: app_id.into(),
            instance_id: instance_id.into(),
        };
        self.inner
            .subscribe(request)
            .await
            .map_err(PandaError::from)
    }
}
