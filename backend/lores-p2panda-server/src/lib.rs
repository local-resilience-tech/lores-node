use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;

use lores_p2panda::{
    IncomingOperation, PandaNode, PandaPublishError, RegionAppTopic, RegionId, RegionTopic,
    SubscriptionError, Topic,
};
use sqlx::SqlitePool;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{Mutex, RwLock, broadcast};
use tokio::time::interval;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Request, Response, Status};

pub mod proto {
    tonic::include_proto!("lores.panda.v1");
}

use proto::{
    ListRegionsRequest, ListRegionsResponse, OperationEvent, PublishRequest, PublishResponse,
    SubscribeRequest,
    panda_server::{Panda, PandaServer},
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
    idempotency_db: SqlitePool,
}

impl PandaService {
    pub async fn new(
        node: Arc<Mutex<Option<Arc<PandaNode>>>>,
        idempotency_db: SqlitePool,
    ) -> Result<Self, sqlx::Error> {
        Self::setup_idempotency_table(&idempotency_db).await?;
        Self::spawn_idempotency_cleanup(&idempotency_db);
        Ok(Self {
            node,
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            idempotency_db,
        })
    }

    async fn setup_idempotency_table(db: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS publish_idempotency_keys (
                topic   BLOB    NOT NULL,
                key     BLOB    NOT NULL,
                seen_at INTEGER NOT NULL,
                PRIMARY KEY (topic, key)
            );
            CREATE INDEX IF NOT EXISTS idx_pik_seen_at
                ON publish_idempotency_keys(seen_at);",
        )
        .execute(db)
        .await?;
        Ok(())
    }

    fn spawn_idempotency_cleanup(db: &SqlitePool) {
        let db = db.clone();
        const CLEANUP_FREQUENCY: Duration = Duration::from_hours(12);
        const RETENTION_SECS: i64 = Duration::from_hours(48).as_secs() as i64;

        tokio::spawn(async move {
            let mut timer = interval(CLEANUP_FREQUENCY);
            loop {
                timer.tick().await;
                let cutoff = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64
                    - RETENTION_SECS;
                let _ = sqlx::query("DELETE FROM publish_idempotency_keys WHERE seen_at < ?")
                    .bind(cutoff)
                    .execute(&db)
                    .await;
            }
        });
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

        let region_app_topic = RegionAppTopic::new(region_id, app_namespace);

        println!(
            "[publish] region={} app_namespace={} payload_bytes={}",
            region_app_topic.region_id,
            region_app_topic.app_namespace,
            req.payload.len()
        );

        // Ensure a subscription exists for this topic so the publisher is
        // available. This is idempotent: if already subscribed the existing
        // broadcast channel is reused.
        let _rx = self
            .ensure_broadcast_subscription(&node, &region_app_topic)
            .await?;

        node.publish_to_region_topic(&region_app_topic, req.payload)
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

        println!(
            "[subscribe] region={} app_namespace={}",
            region_app_topic.region_id, region_app_topic.app_namespace
        );

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
        PandaPublishError::NodeNotStarted => {
            eprintln!("publish error: p2panda node not started");
            Status::unavailable("p2panda node not started")
        }
        PandaPublishError::NoSubscription(t) => {
            let msg = format!("no subscription for topic {}", t.to_hex());
            eprintln!("publish error: {msg}");
            Status::failed_precondition(msg)
        }
        PandaPublishError::Publish(e) => {
            eprintln!("publish error: {e}");
            Status::internal(e.to_string())
        }
        PandaPublishError::AppError(msg) => {
            eprintln!("publish error: {msg}");
            Status::internal(msg)
        }
    }
}

fn subscription_error_to_status(e: SubscriptionError) -> Status {
    match e {
        SubscriptionError::AlreadySubscribed(t) => {
            let msg = format!("already subscribed to topic {:?}", t);
            eprintln!("subscription error: {msg}");
            Status::already_exists(msg)
        }
        SubscriptionError::CreateStream(e) => {
            eprintln!("subscription error: failed to create stream: {e}");
            Status::internal(e.to_string())
        }
    }
}
