use axum_login::permission_required;
use utoipa_axum::router::OpenApiRouter;

use crate::api::auth_api::auth_backend::AppAuthBackend;

mod admin_api;
pub mod auth_api;
pub mod public_api;

pub fn api_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/public_api", public_api::public_api_router())
        .nest("/auth_api", auth_api::auth_api_router())
        .nest(
            "/admin_api",
            admin_api::admin_api_router()
                .route_layer(permission_required!(AppAuthBackend, "admin")),
        )
}
