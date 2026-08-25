use std::sync::Arc;

use lores_p2panda_client::PandaClient;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tokio::sync::{broadcast, watch, Mutex};
use uuid::Uuid;

use crate::consumer::OperationConsumer;
use crate::stores::grpc::GrpcOperationStore;
use crate::stores::local::LocalOperationStore;
use crate::stores::outbox::OutboxStore;
use crate::stores::{OperationStore, StoreError};
use crate::subscription::LiveSubscription;
use crate::types::{AppNodeOperation, NodeEvent};

/// Errors emitted by the node that consumers (e.g. a WebSocket handler) may
/// want to surface directly to users.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeError {
    /// No region has been bound to this app/instance on the remote server.
    RegionNotBound(String),
    /// The remote gRPC server could not be reached or did not respond.
    GrpcUnavailable(String),
}

impl std::fmt::Display for NodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeError::RegionNotBound(msg) => write!(f, "{msg}"),
            NodeError::GrpcUnavailable(msg) => write!(f, "{msg}"),
        }
    }
}

/// The central node handle used by application code.
///
/// Generic over the operation type `Op` — the application supplies its own
/// operation enum and `AppNode` handles serialization, loopback broadcast, and
/// error logging consistently across all operation store backends.
///
/// Construct via the named constructors rather than directly:
/// ```no_run
/// # use lores_app_node::AppNode;
/// # #[derive(Clone, serde::Serialize)] enum Op {}
/// let node = AppNode::<Op>::grpc("http://[::1]:50051".into(), "my-app-id", "my-instance");
/// ```
pub struct AppNode<Op> {
    pub app_id: String,
    pub instance_id: String,
    operation_store: Arc<Mutex<Box<dyn OperationStore>>>,
    consumer: OperationConsumer<Op>,
    error_tx: watch::Sender<Option<NodeError>>,
    node_event_tx: broadcast::Sender<NodeEvent>,
    /// Present only when this node is connected to a gRPC server.
    panda_client: Option<Arc<Mutex<PandaClient>>>,
}

impl<Op> Clone for AppNode<Op> {
    fn clone(&self) -> Self {
        Self {
            app_id: self.app_id.clone(),
            instance_id: self.instance_id.clone(),
            operation_store: self.operation_store.clone(),
            consumer: self.consumer.clone(),
            error_tx: self.error_tx.clone(),
            node_event_tx: self.node_event_tx.clone(),
            panda_client: self.panda_client.clone(),
        }
    }
}

fn make_panda_client(grpc_addr: String) -> Arc<Mutex<PandaClient>> {
    Arc::new(Mutex::new(
        PandaClient::connect_lazy(grpc_addr).expect("failed to build gRPC client"),
    ))
}

impl<Op: Clone + Serialize + Send + 'static> AppNode<Op> {
    fn new(
        app_id: impl Into<String>,
        instance_id: impl Into<String>,
        operation_store: Box<dyn OperationStore>,
        panda_client: Option<Arc<Mutex<PandaClient>>>,
    ) -> Self {
        let (event_tx, _) = broadcast::channel(64);
        let (error_tx, _) = watch::channel(None);
        let (node_event_tx, _) = broadcast::channel(16);
        let consumer = OperationConsumer::new(event_tx);
        Self {
            app_id: app_id.into(),
            instance_id: instance_id.into(),
            operation_store: Arc::new(Mutex::new(operation_store)),
            consumer,
            error_tx,
            node_event_tx,
            panda_client,
        }
    }

    /// Create a local-only `AppNode` backed by a SQLite store.
    ///
    /// Operations are persisted locally and never forwarded to a remote node.
    pub async fn local(
        pool: SqlitePool,
        app_id: impl Into<String>,
        instance_id: impl Into<String>,
    ) -> Result<Self, sqlx::Error> {
        let store = LocalOperationStore::new(pool).await?;
        Ok(Self::new(app_id, instance_id, Box::new(store), None))
    }

    /// Create an `AppNode` that persists to a local SQLite store and forwards
    /// to lores-node via gRPC, using the local row id as an idempotency key.
    ///
    /// If gRPC delivery fails the operation is retained locally for a future
    /// drain attempt.
    pub async fn grpc_with_local(
        pool: SqlitePool,
        grpc_addr: String,
        app_id: impl Into<String>,
        instance_id: impl Into<String>,
    ) -> Result<Self, sqlx::Error> {
        let app_id = app_id.into();
        let instance_id = instance_id.into();
        let local = LocalOperationStore::new(pool).await?;
        let client = make_panda_client(grpc_addr);
        let remote = GrpcOperationStore::new(client.clone(), &app_id, &instance_id);
        let store = OutboxStore::new(local, remote);
        Ok(Self::new(app_id, instance_id, Box::new(store), Some(client)))
    }

    /// Create an `AppNode` connected to an external lores-node via gRPC.
    ///
    /// Uses a lazy connection — no network call until the first publish.
    pub fn grpc(
        grpc_addr: String,
        app_id: impl Into<String>,
        instance_id: impl Into<String>,
    ) -> Self {
        let app_id = app_id.into();
        let instance_id = instance_id.into();
        let client = make_panda_client(grpc_addr);
        let store = GrpcOperationStore::new(client.clone(), &app_id, &instance_id);
        Self::new(app_id, instance_id, Box::new(store), Some(client))
    }

    /// Subscribe to operations published through this node (loopback).
    pub fn subscribe(&self) -> broadcast::Receiver<AppNodeOperation<Op>> {
        self.consumer.subscribe()
    }

    /// Watch the current node error state.
    ///
    /// The receiver immediately reflects the current value, so callers that
    /// subscribe after an error was set will see it right away.
    pub fn subscribe_errors(&self) -> watch::Receiver<Option<NodeError>> {
        self.error_tx.subscribe()
    }

    pub fn subscribe_node_events(&self) -> broadcast::Receiver<NodeEvent> {
        self.node_event_tx.subscribe()
    }

    /// Replay all locally-stored operations, broadcasting each through the
    /// event channel.
    pub async fn replay(&self) -> Result<(), StoreError>
    where
        Op: for<'de> Deserialize<'de>,
    {
        let mut stream = {
            let mut t = self.operation_store.lock().await;
            t.replay().await?
        };

        let count = self.consumer.drain_stream(&mut stream).await?;
        tracing::info!(count, "replay complete");
        Ok(())
    }

    /// Serialize and publish an operation, then broadcast it locally.
    pub async fn publish(&self, operation: &Op) -> Result<(), StoreError> {
        let local_id = Uuid::new_v4();
        let payload = serde_json::to_vec(operation).map_err(|e| {
            tracing::error!("Failed to serialize operation: {e}");
            StoreError::Other(format!("Failed to serialize operation: {e}"))
        })?;
        let mut t = self.operation_store.lock().await;
        let result = t.publish(payload, Some(local_id.to_string())).await?;
        drop(t);

        let app_node_operation = AppNodeOperation::<Op> {
            op: operation.clone(),
            local_operation_id: Some(local_id),
            panda_operation_id: result.operation_id,
            author_node_id: result.node_id,
            timestamp: None,
        };

        self.consumer.send(app_node_operation);
        Ok(())
    }

    /// Drive the remote subscription in a loop, broadcasting incoming operations
    /// and errors to all subscribers.
    ///
    /// Retries on all transient failures with exponential backoff (1 s → 60 s).
    /// The backoff resets whenever the error variant changes (e.g. `GrpcUnavailable`
    /// → `RegionNotBound`). Call it with `tokio::spawn` from your application's `main`.
    pub async fn run(&self)
    where
        Op: for<'de> Deserialize<'de>,
    {
        LiveSubscription::new(
            self.operation_store.clone(),
            self.consumer.clone(),
            self.error_tx.clone(),
            self.node_event_tx.clone(),
            self.panda_client.clone(),
            self.instance_id.clone(),
        )
        .run()
        .await;
    }
}

pub(crate) fn map_store_error(err: StoreError) -> NodeError {
    match err {
        StoreError::RegionNotBound(msg) => NodeError::RegionNotBound(msg),
        StoreError::Other(_) => NodeError::GrpcUnavailable(
            "Could not connect to the LoRes Node for this server".to_string(),
        ),
    }
}
