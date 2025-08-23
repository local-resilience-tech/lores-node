use utoipa_axum::router::OpenApiRouter;

pub mod auth_api;
pub mod public_api;

pub fn api_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/public_api", public_api::public_api_router())
        .nest("/auth_api", auth_api::auth_api_router())
}
