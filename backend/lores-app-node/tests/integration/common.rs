use lores_dev_server::proto::panda_server::PandaServer;
use lores_dev_server::service::DevPandaService;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestOp {
    pub msg: String,
}

/// Start an in-memory dev server on a random free port and return its endpoint.
pub async fn start_dev_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        Server::builder()
            .add_service(PandaServer::new(DevPandaService::new()))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .unwrap();
    });

    format!("http://{addr}")
}

/// Waits up to 5 seconds for the first event, then drains any further events
/// that have already arrived without waiting further. Returns an empty `Vec`
/// on timeout or a closed channel, so callers see a clear assertion failure
/// instead of a panic here.
async fn drain<T: Clone>(rx: &mut tokio::sync::broadcast::Receiver<T>) -> Vec<T> {
    let Ok(Ok(first)) = tokio::time::timeout(std::time::Duration::from_secs(5), rx.recv()).await else {
        return Vec::new();
    };

    let mut events = vec![first];
    while let Ok(event) = rx.try_recv() {
        events.push(event);
    }
    events
}

/// Drains a [`lores_app_node::NodeEvent`] receiver (see [`drain`]).
pub async fn drain_node_events(rx: &mut tokio::sync::broadcast::Receiver<lores_app_node::NodeEvent>) -> Vec<lores_app_node::NodeEvent> {
    drain(rx).await
}

/// Drains an [`lores_app_node::AppNodeOperation`] receiver (see [`drain`]).
pub async fn drain_operations<Op: Clone>(
    rx: &mut tokio::sync::broadcast::Receiver<lores_app_node::AppNodeOperation<Op>>,
) -> Vec<lores_app_node::AppNodeOperation<Op>> {
    drain(rx).await
}

/// A single-connection in-memory SQLite pool, so all queries share one database.
pub async fn memory_pool() -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap()
}

/// Returns an in-memory pool with the `lores_app_operations` schema already
/// created, so tests can seed operations directly.
pub async fn memory_pool_with_schema() -> SqlitePool {
    let pool = memory_pool().await;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS lores_app_operations (
            id         INTEGER PRIMARY KEY AUTOINCREMENT,
            payload    BLOB    NOT NULL,
            created_at INTEGER NOT NULL DEFAULT (unixepoch())
        )",
    )
    .execute(&pool)
    .await
    .unwrap();
    pool
}

/// Insert a raw operation payload into the `lores_app_operations` table.
pub async fn insert_operation_into_db(pool: &SqlitePool, payload: Vec<u8>) {
    sqlx::query("INSERT INTO lores_app_operations (payload) VALUES (?)")
        .bind(payload)
        .execute(pool)
        .await
        .unwrap();
}
