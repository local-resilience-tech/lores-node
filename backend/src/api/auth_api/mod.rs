use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;

mod admin_auth_routes;
mod admin_user_repo;
pub mod auth_backend;
mod auth_repo;
mod node_steward_auth_routes;

pub fn auth_api_router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/admin", admin_auth_routes::router())
        .nest("/node_steward", node_steward_auth_routes::router())
}

#[derive(ToSchema, Serialize)]
struct UserRef {
    user_id: String,
}

impl UserRef {
    pub fn from_backend_user(user: &auth_backend::User) -> Self {
        UserRef {
            user_id: user.id.clone(),
        }
    }
}
