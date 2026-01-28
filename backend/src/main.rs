use axum::{
    http::{header, Method},
    routing::get,
    Extension,
};
use axum_login::AuthManagerLayerBuilder;
use p2panda_core::PublicKey;
use sqlx::SqlitePool;
use time::Duration;
use tokio::sync::mpsc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    // event_handlers::handle_event,,
    api::{
        api_router,
        auth_api::auth_backend::AppAuthBackend,
        public_api::realtime::{self, RealtimeState},
    },
    config::{config::LoresNodeConfig, config_state::LoresNodeConfigState},
    event_handlers::handle_event,
    panda_comms::{
        config::ThisP2PandaNodeRepo,
        lores_events::LoResEvent,
        panda_node_container::{build_public_key_from_hex, PandaNodeContainer},
    },
    static_server::frontend_handler,
};

mod api;
mod config;
mod data;
mod docker;
mod event_handlers;
mod local_apps;
mod panda_comms;
mod static_server;

#[macro_use]
extern crate lazy_static;

#[derive(Clone)]
struct DatabaseState {
    operations_pool: SqlitePool,
    projections_pool: SqlitePool,

    #[allow(dead_code)]
    node_data_pool: SqlitePool,
}

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi()]
    struct ApiDoc;

    // CONFIG PARAMS
    let base_url =
        std::env::var("BASE_URL").unwrap_or_else(|_| "http://lores.localhost:5173".to_string());

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
        .allow_origin(base_url.parse::<header::HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
        .allow_credentials(true);

    // CONFIG
    let config = LoresNodeConfig::load();
    let config_state = LoresNodeConfigState::new(&config);

    // DATABASES
    let projections_pool = data::setup::prepare_projections_database()
        .await
        .expect("Failed to prepare database");
    let node_data_pool = data::setup::prepare_node_data_database()
        .await
        .expect("Failed to prepare node data database");
    let operations_pool = data::setup::prepare_operations_database()
        .await
        .expect("Failed to prepare operation database");

    // SESSION MANAGEMENT
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(30)));
    let backend = AppAuthBackend::new(&config_state, &node_data_pool);
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    // REALTIME COMMS
    let realtime_state = RealtimeState::new();

    // P2PANDA
    let (channel_tx, channel_rx): (mpsc::Sender<LoResEvent>, mpsc::Receiver<LoResEvent>) =
        mpsc::channel(32);
    let panda_container = PandaNodeContainer::new(channel_tx);
    start_panda(&config_state, &panda_container, &operations_pool).await;
    start_panda_event_handler(channel_rx, projections_pool.clone(), realtime_state.clone());

    // ROUTES
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(api_router())
        .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .route("/ws", get(realtime::handler))
        .fallback_service(get(frontend_handler))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .layer(Extension(DatabaseState {
            operations_pool,
            projections_pool,
            node_data_pool,
        }))
        .layer(Extension(config_state))
        .layer(Extension(panda_container))
        .layer(auth_layer)
        .layer(Extension(realtime_state));

    // SERVICE

    let app = router.into_make_service();

    // run our app with hyper, listening globally on port 8200
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8200").await.unwrap();

    println!("Listening on http://localhost:8200, Ctrl+C to stop");

    axum::serve(listener, app).await.unwrap();
}

async fn start_panda(
    config_state: &LoresNodeConfigState,
    container: &PandaNodeContainer,
    operations_pool: &SqlitePool,
) {
    let repo = ThisP2PandaNodeRepo::init();
    let config = config_state.get().await;

    match config.network_name.clone() {
        Some(network_name) => {
            println!("Using network name: {:?}", network_name);
            container.set_network_name(network_name.clone()).await;
        }
        None => {
            println!("No network name set");
        }
    }

    let private_key = match repo.get_or_create_private_key(config_state).await {
        Ok(key) => key,
        Err(e) => {
            println!("Failed to get or create private key: {:?}", e);
            return;
        }
    };

    container.set_private_key(private_key).await;

    let bootstrap_details = repo.get_bootstrap_details(config_state).await;

    let bootstrap_node_id: Option<PublicKey> = match &bootstrap_details {
        Some(details) => build_public_key_from_hex(details.node_id.clone()),
        None => None,
    };
    container.set_bootstrap_node_id(bootstrap_node_id).await;

    if let Err(e) = container.start(operations_pool).await {
        println!("Failed to start P2PandaContainer on liftoff: {:?}", e);
    }
}

fn start_panda_event_handler(
    channel_rx: mpsc::Receiver<LoResEvent>,
    pool: SqlitePool,
    realtime_state: RealtimeState,
) {
    tokio::spawn(async move {
        let mut events_rx = channel_rx;

        // Start the event loop to handle events
        while let Some(event) = events_rx.recv().await {
            handle_event(event, &pool, &realtime_state).await;
        }
    });
}
