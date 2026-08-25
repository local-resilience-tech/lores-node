use std::net::SocketAddr;
use tonic::transport::Server;

use crate::proto::panda_server::PandaServer;
use crate::service::DevPandaService;

mod proto;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr: SocketAddr = std::env::var("PANDA_DEV_SERVER_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:50051".to_string())
        .parse()?;

    println!("starting lores-p2panda-dev-server at {}, press CTRL-C to cancel", addr);

    Server::builder()
        .add_service(PandaServer::new(DevPandaService::new()))
        .serve(addr)
        .await?;

    Ok(())
}
