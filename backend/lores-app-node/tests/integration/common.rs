use lores_p2panda_dev_server::proto::panda_server::PandaServer;
use lores_p2panda_dev_server::service::DevPandaService;
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

/// A single-connection in-memory SQLite pool, so all queries share one database.
pub async fn memory_pool() -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap()
}
