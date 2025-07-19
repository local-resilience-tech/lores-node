use axum::{routing::get, Extension};
use p2panda_core::PublicKey;
use sqlx::SqlitePool;
use tokio::sync::mpsc;
use tower_http::{
    cors::{self, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    admin_api::api_router,
    config::LoresNodeConfig,
    event_handlers::handle_event,
    infra::db,
    panda_comms::{
        config::ThisP2PandaNodeRepo,
        container::{build_public_key_from_hex, P2PandaContainer},
        lores_events::LoResEvent,
    },
    static_server::frontend_handler,
};

mod admin_api;
mod config;
mod event_handlers;
mod infra;
mod panda_comms;
mod projections;
mod static_server;

#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;

    // LOGGING AND TRACING
    tracing_subscriber::fmt()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("axum_tracing_example=error,tower_http=warn"))
                .unwrap(),
        )
        .init();

    // CORS
    let cors = CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_methods(cors::Any)
        .allow_headers(cors::Any);

    // CONFIG
    let mut config = config::LoresNodeConfig::load();

    // DATABASE
    let pool = db::prepare_database()
        .await
        .expect("Failed to prepare database");

    // P2PANDA
    let (channel_tx, channel_rx): (mpsc::Sender<LoResEvent>, mpsc::Receiver<LoResEvent>) =
        mpsc::channel(32);
    let container = P2PandaContainer::new(channel_tx);
    start_panda(&mut config, &container).await;
    start_panda_event_handler(channel_rx, pool.clone());

    // ROUTES
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api", OpenApiRouter::new().merge(api_router()))
        .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .fallback_service(get(frontend_handler))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(pool))
        .layer(Extension(config))
        .layer(Extension(container));

    // SERVICE

    let app = router.into_make_service();

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Listening on http://localhost:8000, Ctrl+C to stop");

    axum::serve(listener, app).await.unwrap();
}

async fn start_panda(config: &mut LoresNodeConfig, container: &P2PandaContainer) {
    let repo = ThisP2PandaNodeRepo::init();

    match config.network_name.clone() {
        Some(network_name) => {
            println!("Using network name: {:?}", network_name);
            container.set_network_name(network_name.clone()).await;
        }
        None => {
            println!("No network name set");
        }
    }

    container
        .set_private_key(repo.get_or_create_private_key(config))
        .await;

    let bootstrap_details = repo.get_bootstrap_details(config);
    let bootstrap_node_id: Option<PublicKey> = match &bootstrap_details {
        Some(details) => build_public_key_from_hex(details.node_id.clone()),
        None => None,
    };
    container.set_bootstrap_node_id(bootstrap_node_id).await;

    if let Err(e) = container.start().await {
        println!("Failed to start P2PandaContainer on liftoff: {:?}", e);
    }
}

fn start_panda_event_handler(channel_rx: mpsc::Receiver<LoResEvent>, pool: SqlitePool) {
    tokio::spawn(async move {
        let mut events_rx = channel_rx;

        // Start the event loop to handle events
        while let Some(event) = events_rx.recv().await {
            handle_event(event, &pool).await;
        }
    });
}
