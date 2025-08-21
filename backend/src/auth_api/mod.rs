use utoipa_axum::router::OpenApiRouter;

mod admin;

pub fn auth_router() -> OpenApiRouter {
    OpenApiRouter::new().nest("/admin", admin::router())
}
