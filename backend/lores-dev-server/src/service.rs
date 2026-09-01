use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

use futures::StreamExt;
use tokio::sync::{RwLock, broadcast};
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Request, Response, Status};
use tracing::{info, warn};

use sha2::{Digest, Sha256};

use lores_p2panda_client::proto::{
    GetNodeRequest, GetNodeResponse, InfoRequest, InfoResponse, OperationEvent, PublishRequest, PublishResponse, SubscribeRequest,
    panda_server::Panda,
};

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
    /// Reverse map from hex node_id to instance_id, used by get_node.
    node_names: Arc<RwLock<HashMap<String, String>>>,
    /// Monotonically increasing counter used to synthesise `operation_id`s.
    counter: Arc<AtomicU64>,
    /// Every operation published to each `app_id`, retained only when tests
    /// need to introspect published operations.
    observed: Option<Arc<RwLock<HashMap<String, Vec<OperationEvent>>>>>,
}

impl DevPandaService {
    /// Create a new `DevPandaService` without operation recording.
    pub fn new() -> Self {
        Self {
            topics: Arc::new(RwLock::new(HashMap::new())),
            node_names: Arc::new(RwLock::new(HashMap::new())),
            counter: Arc::new(AtomicU64::new(1)),
            observed: None,
        }
    }

    /// Create a new `DevPandaService` that retains all published operations
    /// so tests can introspect them.
    pub fn with_operation_recording() -> Self {
        Self {
            topics: Arc::new(RwLock::new(HashMap::new())),
            node_names: Arc::new(RwLock::new(HashMap::new())),
            counter: Arc::new(AtomicU64::new(1)),
            observed: Some(Arc::new(RwLock::new(HashMap::new()))),
        }
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

    /// Return all operations observed for `app_id` so far.
    ///
    /// This is intended for tests that want to assert on what was published
    /// without needing to subscribe before the operations are sent. It returns
    /// an empty vector if operation recording is not enabled.
    pub async fn operations_for_app(&self, app_id: &str) -> Vec<OperationEvent> {
        match &self.observed {
            Some(observed) => observed.read().await.get(app_id).cloned().unwrap_or_default(),
            None => Vec::new(),
        }
    }
}

fn dummy_node_id(instance_id: &str) -> Vec<u8> {
    Sha256::digest(instance_id.as_bytes()).to_vec()
}

fn dummy_region_id(app_id: &str) -> Vec<u8> {
    Sha256::digest(app_id.as_bytes()).to_vec()
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

        let author = dummy_node_id(&req.instance_id);

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        let operation_id = self.next_operation_id();
        let node_id = dummy_node_id(&req.instance_id);
        self.node_names.write().await.insert(hex::encode(&node_id), req.instance_id.clone());

        let event = OperationEvent {
            topic_id: topic_id_from_app_id(&req.app_id),
            author,
            operation_id: operation_id.clone(),
            timestamp,
            payload: req.payload,
        };

        // A lag here means all active subscribers are slow; the dev server
        // simply drops messages when the channel is full, matching the
        // broadcast behaviour of the real server.
        let _ = tx.send(event.clone());

        // Retain a copy for test introspection when enabled.
        if let Some(observed) = &self.observed {
            observed.write().await.entry(req.app_id.clone()).or_default().push(event);
        }

        Ok(Response::new(PublishResponse { operation_id, node_id }))
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

    async fn info(&self, _request: Request<InfoRequest>) -> Result<Response<InfoResponse>, Status> {
        let req = _request.into_inner();
        let node_id = dummy_node_id(&req.instance_id);
        self.node_names.write().await.insert(hex::encode(&node_id), req.instance_id.clone());

        info!(instance_id = %req.instance_id, node_id = %hex::encode(&node_id), "info");

        Ok(Response::new(InfoResponse {
            node_id,
            region: Some(lores_p2panda_client::proto::RegionInfo {
                region_id: dummy_region_id(&req.app_id),
                slug: Some("dev-region".to_string()),
                name: Some("Dev Region".to_string()),
            }),
        }))
    }

    async fn get_node(&self, request: Request<GetNodeRequest>) -> Result<Response<GetNodeResponse>, Status> {
        let req = request.into_inner();
        let name = self.node_names.read().await.get(&req.node_id).cloned();

        info!(node_id = %req.node_id, name = ?name, "get_node");

        Ok(Response::new(GetNodeResponse {
            node_id: req.node_id,
            name,
            domain_on_internet: None,
        }))
    }
}
