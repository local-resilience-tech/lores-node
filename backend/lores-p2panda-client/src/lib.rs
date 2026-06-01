use tonic::transport::Channel;

pub mod proto {
    tonic::include_proto!("lores.panda.v1");
}

use proto::{
    panda_client::PandaClient as TonicPandaClient, ListRegionsRequest, ListRegionsResponse,
    OperationEvent, PublishRequest, PublishResponse, SubscribeRequest,
};
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

    /// Publish an operation to a region+namespace topic.
    ///
    /// Returns only after the operation has been persisted by the remote
    /// p2panda node, guaranteeing eventual propagation to peers.
    pub async fn publish(
        &mut self,
        region_id: [u8; 32],
        app_namespace: impl Into<String>,
        payload: impl Into<Vec<u8>>,
    ) -> Result<Response<PublishResponse>, Status> {
        let request = PublishRequest {
            region_id: region_id.to_vec(),
            app_namespace: app_namespace.into(),
            payload: payload.into(),
        };
        self.inner.publish(request).await
    }

    /// Subscribe to a region+namespace topic and receive a stream of
    /// [`OperationEvent`]s.
    ///
    /// HTTP/2 flow control provides natural backpressure.
    pub async fn subscribe(
        &mut self,
        region_id: [u8; 32],
        app_namespace: impl Into<String>,
    ) -> Result<Response<Streaming<OperationEvent>>, Status> {
        let request = SubscribeRequest {
            region_id: region_id.to_vec(),
            app_namespace: app_namespace.into(),
        };
        self.inner.subscribe(request).await
    }

    /// List all regions the remote node knows about.
    pub async fn list_regions(&mut self) -> Result<Response<ListRegionsResponse>, Status> {
        self.inner.list_regions(ListRegionsRequest {}).await
    }
}
