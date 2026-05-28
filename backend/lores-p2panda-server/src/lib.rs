use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use lores_p2panda::{
    IncomingOperation, PandaNode, PandaPublishError, RegionAppTopic, RegionId, SubscriptionError,
    Topic,
};
use tokio::sync::{broadcast, Mutex, RwLock};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status};

pub mod proto {
    tonic::include_proto!("lores.panda.v1");
}

use proto::{
    panda_server::{Panda, PandaServer},
    ListRegionsRequest, ListRegionsResponse, ListTopicsRequest, ListTopicsResponse, OperationEvent,
    PublishRequest, PublishResponse, SubscribeRequest,
};

/// gRPC service that exposes [`PandaNode`] publish and subscribe over the
/// network.
///
/// A single p2panda network subscription is maintained per topic.  Multiple
/// gRPC subscribers to the same topic share that subscription via a broadcast
/// channel, so the underlying p2panda node only sees one subscriber per topic
/// regardless of how many gRPC clients are connected.
pub struct PandaService {
    node: Arc<Mutex<Option<Arc<PandaNode>>>>,
    /// One broadcast sender per subscribed topic.  Shared across all gRPC
    /// connections so the p2panda-level subscription is created only once.
    subscriptions: Arc<RwLock<HashMap<Topic, broadcast::Sender<IncomingOperation>>>>,
}

impl PandaService {
    pub fn new(node: Arc<Mutex<Option<Arc<PandaNode>>>>) -> Self {
        Self {
            node,
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn into_server(self) -> PandaServer<Self> {
        PandaServer::new(self)
    }

    /// Returns a broadcast receiver for the topic derived from `region_id` and
    /// `app_namespace`, creating the underlying p2panda subscription and
    /// forwarding task the first time this topic is seen.
    async fn ensure_broadcast_subscription(
        &self,
        node: &PandaNode,
        region_app_topic: &RegionAppTopic,
    ) -> Result<broadcast::Receiver<IncomingOperation>, Status> {
        let topic = region_app_topic.p2panda_topic();
        let mut subs = self.subscriptions.write().await;

        if let Some(tx) = subs.get(&topic) {
            return Ok(tx.subscribe());
        }

        let (broadcast_tx, broadcast_rx) = broadcast::channel::<IncomingOperation>(128);
        let (incoming_tx, mut incoming_rx) = tokio::sync::mpsc::channel::<IncomingOperation>(128);

        node.subscribe_to_app_topic(region_app_topic, incoming_tx)
            .await
            .map_err(subscription_error_to_status)?;

        let fwd_tx = broadcast_tx.clone();
        tokio::spawn(async move {
            while let Some(op) = incoming_rx.recv().await {
                let _ = fwd_tx.send(op);
            }
        });

        subs.insert(topic, broadcast_tx);
        Ok(broadcast_rx)
    }
}

#[tonic::async_trait]
impl Panda for PandaService {
    async fn publish(
        &self,
        request: Request<PublishRequest>,
    ) -> Result<Response<PublishResponse>, Status> {
        let req = request.into_inner();

        let region_bytes: [u8; 32] = req
            .region_id
            .try_into()
            .map_err(|_| Status::invalid_argument("region_id must be exactly 32 bytes"))?;

        let region_id = RegionId::from(region_bytes);
        let app_namespace = req.app_namespace;

        let node_lock = self.node.lock().await;
        let node = node_lock
            .as_ref()
            .ok_or_else(|| Status::unavailable("p2panda node is not yet started"))?
            .clone();
        drop(node_lock);

        node.publish_to_app_topic(&RegionAppTopic::new(region_id, app_namespace), req.payload)
            .await
            .map_err(publish_error_to_status)?;

        Ok(Response::new(PublishResponse {}))
    }

    type SubscribeStream =
        Pin<Box<dyn tokio_stream::Stream<Item = Result<OperationEvent, Status>> + Send + 'static>>;

    async fn subscribe(
        &self,
        request: Request<SubscribeRequest>,
    ) -> Result<Response<Self::SubscribeStream>, Status> {
        let req = request.into_inner();

        let region_bytes: [u8; 32] = req
            .region_id
            .try_into()
            .map_err(|_| Status::invalid_argument("region_id must be exactly 32 bytes"))?;

        let region_id = RegionId::from(region_bytes);
        let app_namespace = req.app_namespace;

        let node_lock = self.node.lock().await;
        let node = node_lock
            .as_ref()
            .ok_or_else(|| Status::unavailable("p2panda node is not yet started"))?
            .clone();
        drop(node_lock);

        let region_app_topic = RegionAppTopic::new(region_id, app_namespace);

        // Under a write lock, ensure a p2panda subscription exists for this
        // topic and return a broadcast receiver for it.
        let receiver = self
            .ensure_broadcast_subscription(&node, &region_app_topic)
            .await?;

        // Record the region/namespace so ListRegions can report it.
        node.register_region(region_app_topic.region_id).await;

        let stream = BroadcastStream::new(receiver).filter_map(|result| match result {
            Ok(op) => Some(Ok(incoming_to_event(op))),
            // Lagged means the consumer fell behind; skip the lost messages.
            Err(_lagged) => None,
        });

        Ok(Response::new(Box::pin(stream)))
    }

    async fn list_regions(
        &self,
        _request: Request<ListRegionsRequest>,
    ) -> Result<Response<ListRegionsResponse>, Status> {
        let node_lock = self.node.lock().await;
        let node = node_lock
            .as_ref()
            .ok_or_else(|| Status::unavailable("p2panda node is not yet started"))?
            .clone();
        drop(node_lock);

        let region_ids = node
            .get_regions()
            .await
            .into_iter()
            .map(|id| id.to_bytes().to_vec())
            .collect();

        Ok(Response::new(ListRegionsResponse { region_ids }))
    }

    async fn list_topics(
        &self,
        _request: Request<ListTopicsRequest>,
    ) -> Result<Response<ListTopicsResponse>, Status> {
        let node_lock = self.node.lock().await;
        let node = node_lock
            .as_ref()
            .ok_or_else(|| Status::unavailable("p2panda node is not yet started"))?
            .clone();
        drop(node_lock);

        let topic_ids = node
            .get_subscribed_topics()
            .await
            .into_iter()
            .map(|t| t.to_bytes().to_vec())
            .collect();

        Ok(Response::new(ListTopicsResponse { topic_ids }))
    }
}

fn incoming_to_event(op: IncomingOperation) -> OperationEvent {
    OperationEvent {
        topic_id: op.topic.to_bytes().to_vec(),
        author: op.author.as_bytes().to_vec(),
        operation_id: op.operation_id.as_bytes().to_vec(),
        timestamp: op.timestamp,
        payload: op.bytes,
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

fn subscription_error_to_status(e: SubscriptionError) -> Status {
    match e {
        SubscriptionError::AlreadySubscribed(t) => {
            Status::already_exists(format!("already subscribed to topic {:?}", t))
        }
        SubscriptionError::CreateStream(e) => Status::internal(e.to_string()),
    }
}
