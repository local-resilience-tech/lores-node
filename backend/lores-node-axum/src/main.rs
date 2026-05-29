use axum::{
    http::{header, Method},
    routing::get,
    Extension,
};
use axum_login::AuthManagerLayerBuilder;
use sqlx::SqlitePool;
use std::env;
use time::Duration;
use tokio::sync::mpsc;
use tonic::transport::Server as GrpcServer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tower_sessions::{Expiry, SessionManagerLayer};
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
    panda_comms::{
        lores_events::LoResEvent, start_panda, start_panda_event_handler, PandaContainer,
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

    // SESSION MANAGEMENT
    let session_store = data::setup::prepare_session_store(&node_data_pool)
        .await
        .expect("Failed to prepare session store");
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
    let panda_container = PandaContainer::new(channel_tx);
    start_panda_event_handler(channel_rx, projections_pool.clone(), realtime_state.clone());
    start_panda(&config_state, &panda_container, &projections_pool).await;

    // GRPC SERVER
    let grpc_port = env::var("GRPC_PORT").unwrap_or_else(|_| "50051".to_string());
    let grpc_addr = format!("0.0.0.0:{}", grpc_port)
        .parse()
        .expect("valid gRPC bind address");
    let panda_publish = lores_p2panda_server::PandaService::new(panda_container.node_arc());
    tokio::spawn(async move {
        println!("gRPC listening on {}", grpc_addr);
        GrpcServer::builder()
            .add_service(panda_publish.into_server())
            .serve(grpc_addr)
            .await
            .expect("gRPC server failed");
    });

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
            projections_pool,
            node_data_pool,
        }))
        .layer(Extension(config_state))
        .layer(Extension(panda_container))
        .layer(auth_layer)
        .layer(Extension(realtime_state));

    // SERVICE

    let app = router.into_make_service();

    let port = env::var("HTTP_PORT").unwrap_or_else(|_| "8200".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("Listening on http://localhost:{}, Ctrl+C to stop", port);

    axum::serve(listener, app).await.unwrap();
}
