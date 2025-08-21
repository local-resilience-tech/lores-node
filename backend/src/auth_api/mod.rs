use utoipa_axum::router::OpenApiRouter;

mod admin_routes;
mod admin_user_repo;
pub mod auth_backend;
mod auth_repo;

pub fn auth_router() -> OpenApiRouter {
    OpenApiRouter::new().nest("/admin", admin_routes::router())
}
