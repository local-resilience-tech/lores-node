use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tokio::sync::{broadcast, watch, Mutex};

use crate::consumer::OperationConsumer;
use crate::grpc::GrpcOperationStore;
use crate::local::LocalOperationStore;
use crate::outbox::OutboxStore;
use crate::store::{OperationStore, StoreError};
use crate::subscription::LiveSubscription;

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

#[derive(Clone, Serialize, Deserialize)]
pub struct AppNodeOperation<Op> {
    pub op: Op,
    /// 32-byte p2panda author public key. `None` for locally-published operations.
    pub author: Option<Vec<u8>>,
    /// 32-byte p2panda operation hash. `None` for locally-published operations.
    pub operation_id: Option<Vec<u8>>,
    /// Unix timestamp in milliseconds. `None` for locally-published operations.
    pub timestamp: Option<u64>,
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
}

impl<Op> Clone for AppNode<Op> {
    fn clone(&self) -> Self {
        Self {
            app_id: self.app_id.clone(),
            instance_id: self.instance_id.clone(),
            operation_store: self.operation_store.clone(),
            consumer: self.consumer.clone(),
            error_tx: self.error_tx.clone(),
        }
    }
}

impl<Op: Clone + Serialize + Send + 'static> AppNode<Op> {
    fn new(
        app_id: impl Into<String>,
        instance_id: impl Into<String>,
        operation_store: Box<dyn OperationStore>,
    ) -> Self {
        let (event_tx, _) = broadcast::channel(64);
        let (error_tx, _) = watch::channel(None);
        let consumer = OperationConsumer::new(event_tx);
        Self {
            app_id: app_id.into(),
            instance_id: instance_id.into(),
            operation_store: Arc::new(Mutex::new(operation_store)),
            consumer,
            error_tx,
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
        Ok(Self::new(app_id, instance_id, Box::new(store)))
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
        let remote = GrpcOperationStore::connect_lazy(grpc_addr, &app_id, &instance_id)
            .expect("failed to build gRPC opereration store");
        let store = OutboxStore::new(local, remote);
        Ok(Self::new(app_id, instance_id, Box::new(store)))
    }

    /// Create an `AppNode` connected to an external lores-node via gRPC.
    ///
    /// Uses a lazy connection — no network call until the first publish.
    pub fn grpc(grpc_addr: String, app_id: impl Into<String>, instance_id: impl Into<String>) -> Self {
        let app_id = app_id.into();
        let instance_id = instance_id.into();
        let operation_store = GrpcOperationStore::connect_lazy(grpc_addr, &app_id, &instance_id)
            .expect("failed to build gRPC operation store");
        Self::new(app_id, instance_id, Box::new(operation_store))
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
        let payload = serde_json::to_vec(operation).map_err(|e| {
            tracing::error!("Failed to serialize operation: {e}");
            StoreError::Other(format!("Failed to serialize operation: {e}"))
        })?;
        let mut t = self.operation_store.lock().await;
        t.publish(payload, None).await?;
        drop(t);

        let app_node_operation = AppNodeOperation::<Op> {
            op: operation.clone(),
            author: None,
            operation_id: None,
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
        )
        .run()
        .await;
    }
}

pub(crate) fn map_store_error(err: StoreError) -> NodeError {
    match err {
        StoreError::RegionNotBound(msg) => NodeError::RegionNotBound(msg),
        StoreError::Other(_) => {
            NodeError::GrpcUnavailable("Could not connect to the LoRes Node for this server".to_string())
        }
    }
}
