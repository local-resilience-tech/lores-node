use std::sync::Arc;

use lores_p2panda::{PandaNode, PandaPublishError, Topic};
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

pub mod proto {
    tonic::include_proto!("lores.panda.v1");
}

use proto::{
    panda_publish_server::{PandaPublish, PandaPublishServer},
    PublishRequest, PublishResponse,
};

/// gRPC service that exposes [`PandaNode::publish`] over the network.
///
/// The inner node is held behind an `Option` because the p2panda node may not
/// yet be started when the gRPC server begins accepting connections.  A
/// [`Status::unavailable`] error is returned to callers until the node is ready.
pub struct PandaPublishService {
    node: Arc<Mutex<Option<Arc<PandaNode>>>>,
}

impl PandaPublishService {
    pub fn new(node: Arc<Mutex<Option<Arc<PandaNode>>>>) -> Self {
        Self { node }
    }

    pub fn into_server(self) -> PandaPublishServer<Self> {
        PandaPublishServer::new(self)
    }
}

#[tonic::async_trait]
impl PandaPublish for PandaPublishService {
    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<PublishResponse>, Status> {
        let req = request.into_inner();

        let topic_bytes: [u8; 32] = req
            .topic_id
            .try_into()
            .map_err(|_| Status::invalid_argument("topic_id must be exactly 32 bytes"))?;

        let topic = Topic::from(topic_bytes);

        let node_lock = self.node.lock().await;
        let node = node_lock
            .as_ref()
            .ok_or_else(|| Status::unavailable("p2panda node is not yet started"))?
            .clone();
        drop(node_lock);

        node.publish(topic, req.payload)
            .await
            .map_err(publish_error_to_status)?;

        Ok(Response::new(PublishResponse {}))
    }
}

fn publish_error_to_status(e: PandaPublishError) -> Status {
    match e {
        PandaPublishError::NodeNotStarted => Status::unavailable("p2panda node not started"),
        PandaPublishError::NoSubscription(t) => {
            Status::failed_precondition(format!("no subscription for topic {:?}", t))
        }
        PandaPublishError::Publish(e) => Status::internal(e.to_string()),
        PandaPublishError::AppError(msg) => Status::internal(msg),
    }
}
