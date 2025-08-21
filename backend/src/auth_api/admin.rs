use axum::{http::StatusCode, response::IntoResponse};
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(generate_admin_password))
}

#[utoipa::path(post, path = "/", responses(
    (status = CREATED, body = ()),
    (status = BAD_REQUEST, body = ()),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn generate_admin_password() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, ())
}
