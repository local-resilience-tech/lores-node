#![doc = include_str!("../README.md")]

use std::net::SocketAddr;

use tonic::transport::Server;

use crate::service::DevPandaService;
use lores_p2panda_client::proto::panda_server::PandaServer;

pub use lores_p2panda_client::proto;
pub mod service;

/// Start the dev gRPC server, reading `PANDA_DEV_SERVER_ADDR` from the
/// environment and defaulting to `127.0.0.1:50051`.
pub async fn run_from_env() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = std::env::var("PANDA_DEV_SERVER_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:50051".to_string())
        .parse()?;

    println!("starting lores-dev-server at {addr}, press CTRL-C to cancel");

    Server::builder()
        .add_service(PandaServer::new(DevPandaService::new()))
        .serve(addr)
        .await?;

    Ok(())
}
