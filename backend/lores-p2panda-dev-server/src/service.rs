use std::collections::HashMap;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::SystemTime;

use futures::StreamExt;
use tokio::sync::{broadcast, RwLock};
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Request, Response, Status};
use tracing::{info, warn};

use crate::proto::{panda_server::Panda, OperationEvent, PublishRequest, PublishResponse, SubscribeRequest};

/// In-memory dev server for the lores-p2panda gRPC API.
///
/// Operations are broadcast to all live subscribers for the same `app_id`.
/// Nothing is persisted, and idempotency keys are ignored. This is intended
/// only for local multi-instance development and testing.
#[derive(Clone)]
pub struct DevPandaService {
    /// One broadcast channel per `app_id`. All subscribers to the same app
    /// share the same channel so they see each other's operations.
    topics: Arc<RwLock<HashMap<String, broadcast::Sender<OperationEvent>>>>,
    /// Stable author id (32-byte, incrementing) assigned per `instance_id`.
    authors: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    /// Monotonically increasing counter used to synthesise `operation_id`s.
    counter: Arc<AtomicU64>,
    /// Monotonically increasing counter used to assign author ids.
    author_counter: Arc<AtomicU64>,
}

impl DevPandaService {
    pub fn new() -> Self {
        Self {
            topics: Arc::new(RwLock::new(HashMap::new())),
            authors: Arc::new(RwLock::new(HashMap::new())),
            counter: Arc::new(AtomicU64::new(1)),
            author_counter: Arc::new(AtomicU64::new(1)),
        }
    }

    async fn author_id_for(&self, instance_id: &str) -> Vec<u8> {
        {
            let authors = self.authors.read().await;
            if let Some(id) = authors.get(instance_id) {
                return id.clone();
            }
        }
        let mut authors = self.authors.write().await;
        // Re-check after acquiring write lock.
        authors
            .entry(instance_id.to_string())
            .or_insert_with(|| {
                let n = self.author_counter.fetch_add(1, Ordering::SeqCst);
                let mut bytes = vec![0u8; 32];
                bytes[24..32].copy_from_slice(&n.to_be_bytes());
                bytes
            })
            .clone()
    }

    async fn topic_tx(&self, app_id: &str) -> broadcast::Sender<OperationEvent> {
        let topics = self.topics.read().await;
        if let Some(tx) = topics.get(app_id) {
            return tx.clone();
        }
        drop(topics);

        let mut topics = self.topics.write().await;
        topics
            .entry(app_id.to_string())
            .or_insert_with(|| broadcast::channel(256).0)
            .clone()
    }

    fn next_operation_id(&self) -> Vec<u8> {
        let n = self.counter.fetch_add(1, Ordering::SeqCst);
        let mut bytes = vec![0u8; 32];
        bytes[24..32].copy_from_slice(&n.to_be_bytes());
        bytes
    }
}

fn topic_id_from_app_id(app_id: &str) -> Vec<u8> {
    let mut topic_id = vec![0u8; 32];
    let bytes = app_id.as_bytes();
    let len = bytes.len().min(32);
    topic_id[..len].copy_from_slice(&bytes[..len]);
    topic_id
}

#[tonic::async_trait]
impl Panda for DevPandaService {
    async fn publish(&self, request: Request<PublishRequest>) -> Result<Response<PublishResponse>, Status> {
        let req = request.into_inner();

        info!(
            app_id = %req.app_id,
            instance_id = %req.instance_id,
            payload_bytes = req.payload.len(),
            "publish"
        );

        let tx = self.topic_tx(&req.app_id).await;

        let author = self.author_id_for(&req.instance_id).await;

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let event = OperationEvent {
            topic_id: topic_id_from_app_id(&req.app_id),
            author,
            operation_id: self.next_operation_id(),
            timestamp,
            payload: req.payload,
        };

        // A lag here means all active subscribers are slow; the dev server
        // simply drops messages when the channel is full, matching the
        // broadcast behaviour of the real server.
        let _ = tx.send(event);

        Ok(Response::new(PublishResponse {}))
    }

    type SubscribeStream = Pin<Box<dyn tokio_stream::Stream<Item = Result<OperationEvent, Status>> + Send + 'static>>;

    async fn subscribe(&self, request: Request<SubscribeRequest>) -> Result<Response<Self::SubscribeStream>, Status> {
        let req = request.into_inner();

        info!(app_id = %req.app_id, instance_id = %req.instance_id, "subscribe");

        let tx = self.topic_tx(&req.app_id).await;
        let rx = tx.subscribe();

        let stream = BroadcastStream::new(rx).filter_map(|result| async move {
            match result {
                Ok(event) => Some(Ok(event)),
                Err(_lagged) => {
                    warn!("subscriber lagged behind; dropping messages");
                    None
                }
            }
        });

        Ok(Response::new(Box::pin(stream)))
    }
}
