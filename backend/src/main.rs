use axum::Extension;
use p2panda_core::PublicKey;
use sqlx::SqlitePool;
use tokio::sync::mpsc;
use tower_http::cors::{self, CorsLayer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    api::api_router,
    infra::db,
    panda_comms::{
        container::{build_public_key_from_hex, P2PandaContainer},
        lores_events::LoResEvent,
    },
    repos::this_p2panda_node::ThisP2PandaNodeRepo,
};

mod api;
mod infra;
mod panda_comms;
mod repos;
mod routes;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;

    // CORS
    let cors = CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_headers(cors::Any);

    // DATABASE
    let pool = db::prepare_database()
        .await
        .expect("Failed to prepare database");

    // P2PANDA
    let (channel_tx, channel_rx): (mpsc::Sender<LoResEvent>, mpsc::Receiver<LoResEvent>) =
        mpsc::channel(32);
    let container = P2PandaContainer::new(channel_tx);
    start_panda(&pool, &container).await;

    // ROUTES
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api", OpenApiRouter::new().merge(api_router()))
        .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .layer(cors)
        .layer(Extension(pool))
        .layer(Extension(container));

    // SERVICE

    let app = router.into_make_service();

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Listening on http://localhost:8000, Ctrl+C to stop");

    axum::serve(listener, app).await.unwrap();
}

async fn start_panda(pool: &SqlitePool, container: &P2PandaContainer) {
    let repo = ThisP2PandaNodeRepo::init();

    match repo.get_network_name(pool).await {
        Ok(network_name) => {
            if let Some(network_name) = network_name {
                println!("Got network name: {:?}", network_name);
                container.set_network_name(network_name).await;
            }
        }
        Err(_) => {
            println!("Failed to get network name");
        }
    }

    match repo.get_or_create_private_key(pool).await {
        Ok(private_key) => {
            println!("Got private key");
            container.set_private_key(private_key).await;
        }
        Err(_) => {
            println!("Failed to get private key");
        }
    }

    let bootstrap_details = repo.get_bootstrap_details(pool).await.unwrap();
    let bootstrap_node_id: Option<PublicKey> = match &bootstrap_details {
        Some(details) => build_public_key_from_hex(details.node_id.clone()),
        None => None,
    };
    container.set_bootstrap_node_id(bootstrap_node_id).await;

    if let Err(e) = container.start().await {
        println!("Failed to start P2PandaContainer on liftoff: {:?}", e);
    }
}
