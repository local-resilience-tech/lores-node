use utoipa_axum::router::OpenApiRouter;

mod admin;
pub mod auth_backend;
mod auth_repo;

pub fn auth_router() -> OpenApiRouter {
    OpenApiRouter::new().nest("/admin", admin::router())
}
