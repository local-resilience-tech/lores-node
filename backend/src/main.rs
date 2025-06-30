use axum::Extension;
use tower_http::cors::{self, CorsLayer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{api::api_router, infra::db};

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

    // ROUTES
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api", OpenApiRouter::new().merge(api_router()))
        .split_for_parts();

    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
        .layer(cors)
        .layer(Extension(pool));

    // SERVICE
    let app = router.into_make_service();

    // run our app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("Listening on http://localhost:8000, Ctrl+C to stop");

    axum::serve(listener, app).await.unwrap();
}
