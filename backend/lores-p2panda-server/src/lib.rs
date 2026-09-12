use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tracing::{info, warn};

use lores_p2panda::{IncomingOperation, PandaNode, PandaPublishError, RegionAppTopic, RegionId, RegionTopic, SubscriptionError, Topic};
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock, broadcast};
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Request, Response, Status};

mod idempotency_store;
use idempotency_store::IdempotencyStore;

mod instance_notifier;
use instance_notifier::InstanceNotifier;

/// Configuration for the publish idempotency deduplication store.
pub struct IdempotencyConfig {
    /// How often the cleanup task runs to remove expired keys.
    pub cleanup_frequency: Duration,
    /// How long keys are retained before being eligible for cleanup.
    pub retention: Duration,
}

impl Default for IdempotencyConfig {
    fn default() -> Self {
        Self {
            cleanup_frequency: Duration::from_hours(12),
            retention: Duration::from_hours(48),
        }
    }
}

pub mod proto {
    tonic::include_proto!("lores.panda.v2");
}

use proto::{
    GetNodeRequest, GetNodeResponse, InfoRequest, InfoResponse, OperationEvent, PublishRequest, PublishResponse, SubscribeRequest,
    panda_server::{Panda, PandaServer},
};

/// Error returned by the [`ResolveRegion`] callback.
#[derive(Debug)]
pub enum ResolveRegionIdError {
    /// No region is bound to the given app/instance.
    NotFound,
    /// The stored binding is corrupt or otherwise unusable.
    Internal,
}

/// Region identity and metadata returned by the [`ResolveRegion`] callback.
#[derive(Debug, Clone)]
pub struct ResolvedRegion {
    pub region_id: RegionId,
    pub slug: Option<String>,
    pub name: Option<String>,
}

/// Identifies the app and instance making a gRPC request.
#[derive(Debug, Clone)]
pub struct AppInstanceIds {
    pub app_id: String,
    pub instance_id: String,
}

/// Async callback that resolves region identity and metadata for an app instance.
/// The owner (e.g. `lores-node-axum`) supplies this when constructing
/// [`PandaService`].
pub type ResolveRegionId =
    Arc<dyn Fn(AppInstanceIds) -> Pin<Box<dyn Future<Output = Result<ResolvedRegion, ResolveRegionIdError>> + Send>> + Send + Sync>;

/// Node information returned by the [`ResolveNodeInfo`] callback.
#[derive(Debug, Clone)]
pub struct NodeInfo {
    pub node_id: String,
    pub name: Option<String>,
    pub domain_on_internet: Option<String>,
}

/// Async callback that looks up node metadata within a region.
/// Returns `None` when the node exists but has no enrichment data.
/// Returns `Err(ResolveRegionIdError::NotFound)` when the node is not in the region.
pub type ResolveNodeInfo =
    Arc<dyn Fn(AppInstanceIds, String) -> Pin<Box<dyn Future<Output = Result<NodeInfo, ResolveRegionIdError>> + Send>> + Send + Sync>;

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
    idempotency: IdempotencyStore,
    instance_notifier: InstanceNotifier,
    resolve_region_id: ResolveRegionId,
    resolve_node_info: ResolveNodeInfo,
}

impl PandaService {
    pub async fn new(
        node: Arc<Mutex<Option<Arc<PandaNode>>>>,
        db: SqlitePool,
        idempotency_config: Option<IdempotencyConfig>,
        on_instance_seen: Arc<dyn Fn(String, String) + Send + Sync>,
        resolve_region_id: ResolveRegionId,
        resolve_node_info: ResolveNodeInfo,
    ) -> Result<Self, sqlx::Error> {
        let config = idempotency_config.unwrap_or_default();
        let idempotency = IdempotencyStore::new(db, config.cleanup_frequency, config.retention).await?;
        Ok(Self {
            node,
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            idempotency,
            instance_notifier: InstanceNotifier::new(on_instance_seen),
            resolve_region_id,
            resolve_node_info,
        })
    }

    pub fn into_server(self) -> PandaServer<Self> {
        PandaServer::new(self)
    }

    /// Returns a broadcast receiver for the topic derived from `region_id` and
    /// `app_id`, creating the underlying p2panda subscription and
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

        node.subscribe_to_region_topic(region_app_topic, incoming_tx)
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
    async fn publish(&self, request: Request<PublishRequest>) -> Result<Response<PublishResponse>, Status> {
        let req = request.into_inner();

        let ids = AppInstanceIds {
            app_id: req.app_id,
            instance_id: req.instance_id,
        };

        let region = (self.resolve_region_id)(ids.clone())
            .await
            .map_err(|e| resolve_region_error_to_status(e, &ids))?;

        let node_lock = self.node.lock().await;
        let node = node_lock
            .as_ref()
            .ok_or_else(|| Status::unavailable("p2panda node is not yet started"))?
            .clone();
        drop(node_lock);

        let region_app_topic = RegionAppTopic::new(region.region_id, ids.app_id);

        info!(
            "[publish] region={} app_id={} payload_bytes={}",
            region_app_topic.region_id,
            region_app_topic.app_id,
            req.payload.len()
        );

        // If the client supplied an idempotency key, return early on duplicate.
        if let Some(existing_id) = self.idempotency.check_duplicate(&region_app_topic, &req.idempotency_key).await? {
            info!("[publish] duplicate idempotency key, returning existing operation_id");
            let node_id = node.public_key.as_bytes().to_vec();
            return Ok(Response::new(PublishResponse {
                operation_id: existing_id,
                node_id,
            }));
        }

        // Ensure a subscription exists for this topic so the publisher is
        // available. This is idempotent: if already subscribed the existing
        // broadcast channel is reused.
        let _rx = self.ensure_broadcast_subscription(&node, &region_app_topic).await?;

        let operation_id = node
            .publish_to_region_topic(&region_app_topic, req.payload)
            .await
            .map_err(publish_error_to_status)?;

        // Record the key only after a successful publish so a publish failure
        // does not burn the key — the client can safely retry.
        self.idempotency
            .record(&region_app_topic, &req.idempotency_key, operation_id.as_bytes())
            .await?;

        self.instance_notifier.notify(&region_app_topic.app_id, &ids.instance_id).await;

        Ok(Response::new(PublishResponse {
            operation_id: operation_id.as_bytes().to_vec(),
            node_id: node.public_key.as_bytes().to_vec(),
        }))
    }

    type SubscribeStream = Pin<Box<dyn tokio_stream::Stream<Item = Result<OperationEvent, Status>> + Send + 'static>>;

    async fn subscribe(&self, request: Request<SubscribeRequest>) -> Result<Response<Self::SubscribeStream>, Status> {
        let req = request.into_inner();

        let ids = AppInstanceIds {
            app_id: req.app_id,
            instance_id: req.instance_id,
        };

        let region = (self.resolve_region_id)(ids.clone())
            .await
            .map_err(|e| resolve_region_error_to_status(e, &ids))?;

        let node_lock = self.node.lock().await;
        let node = node_lock
            .as_ref()
            .ok_or_else(|| Status::unavailable("p2panda node is not yet started"))?
            .clone();
        drop(node_lock);

        let region_app_topic = RegionAppTopic::new(region.region_id, ids.app_id);

        info!(
            "[subscribe] region={} app_id={}",
            region_app_topic.region_id, region_app_topic.app_id
        );

        self.instance_notifier.notify(&region_app_topic.app_id, &ids.instance_id).await;

        // Under a write lock, ensure a p2panda subscription exists for this
        // topic and return a broadcast receiver for it.
        let receiver = self.ensure_broadcast_subscription(&node, &region_app_topic).await?;

        // Record the region/namespace so ListRegions can report it.
        node.register_region(region_app_topic.region_id).await;

        let stream = BroadcastStream::new(receiver).filter_map(|result| match result {
            Ok(op) => Some(Ok(incoming_to_event(op))),
            // Lagged means the consumer fell behind; skip the lost messages.
            Err(_lagged) => None,
        });

        Ok(Response::new(Box::pin(stream)))
    }

    async fn info(&self, request: Request<InfoRequest>) -> Result<Response<InfoResponse>, Status> {
        let req = request.into_inner();
        let node_lock = self.node.lock().await;
        let node = node_lock
            .as_ref()
            .ok_or_else(|| Status::unavailable("p2panda node is not yet started"))?;
        let node_id = node.public_key.as_bytes().to_vec();
        drop(node_lock);
        let ids = AppInstanceIds {
            app_id: req.app_id,
            instance_id: req.instance_id,
        };
        let region = (self.resolve_region_id)(ids.clone())
            .await
            .map_err(|e| resolve_region_error_to_status(e, &ids))?;
        Ok(Response::new(InfoResponse {
            node_id,
            region: Some(proto::RegionInfo {
                region_id: <[u8; 32]>::from(region.region_id).to_vec(),
                slug: region.slug,
                name: region.name,
            }),
        }))
    }

    async fn get_node(&self, request: Request<GetNodeRequest>) -> Result<Response<GetNodeResponse>, Status> {
        let req = request.into_inner();
        let ids = AppInstanceIds {
            app_id: req.app_id,
            instance_id: req.instance_id,
        };
        let info = (self.resolve_node_info)(ids.clone(), req.node_id.clone())
            .await
            .map_err(|e| resolve_region_error_to_status(e, &ids))?;
        Ok(Response::new(GetNodeResponse {
            node_id: info.node_id,
            name: info.name,
            domain_on_internet: info.domain_on_internet,
        }))
    }
}

fn incoming_to_event(op: IncomingOperation) -> OperationEvent {
    OperationEvent {
        topic_id: op.topic.to_bytes().to_vec(),
        author: op.author.as_bytes().to_vec(),
        operation_id: op.operation_id.as_bytes().to_vec(),
        timestamp: op.received_timestamp,
        payload: op.bytes,
    }
}

fn publish_error_to_status(e: PandaPublishError) -> Status {
    match e {
        PandaPublishError::NodeNotStarted => {
            warn!("publish error: p2panda node not started");
            Status::unavailable("p2panda node not started")
        }
        PandaPublishError::NoSubscription(t) => {
            let msg = format!("no subscription for topic {}", t.to_hex());
            warn!("publish error: {msg}");
            Status::failed_precondition(msg)
        }
        PandaPublishError::Publish(e) => {
            warn!("publish error: {e}");
            Status::internal(e.to_string())
        }
        PandaPublishError::EphemeralPublish(e) => {
            warn!("ephemeral publish error: {e}");
            Status::internal(e.to_string())
        }
        PandaPublishError::AppError(msg) => {
            warn!("publish error: {msg}");
            Status::internal(msg)
        }
    }
}

fn subscription_error_to_status(e: SubscriptionError) -> Status {
    match e {
        SubscriptionError::AlreadySubscribed(t) => {
            let msg = format!("already subscribed to topic {:?}", t);
            warn!("subscription error: {msg}");
            Status::already_exists(msg)
        }
        SubscriptionError::CreateStream(e) => {
            warn!("subscription error: failed to create stream: {e}");
            Status::internal(e.to_string())
        }
    }
}

fn resolve_region_error_to_status(e: ResolveRegionIdError, ids: &AppInstanceIds) -> Status {
    match e {
        ResolveRegionIdError::NotFound => Status::not_found(format!(
            "No region bound to app '{}' instance '{}'. Use your lores-node installation to bind this app to a region, matching both the app name and instance ID.",
            ids.app_id, ids.instance_id,
        )),
        ResolveRegionIdError::Internal => Status::internal(format!(
            "Failed to resolve region for app instance. This may be an internal server issue with lores-node.",
        )),
    }
}
