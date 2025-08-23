use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;

mod admin_routes;
mod admin_user_repo;
pub mod auth_backend;
mod auth_repo;

pub fn auth_router() -> OpenApiRouter {
    OpenApiRouter::new().nest("/admin", admin_routes::router())
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
