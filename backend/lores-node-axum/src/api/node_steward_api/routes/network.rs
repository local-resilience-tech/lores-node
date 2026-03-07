use axum::{http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(add_bootstrap_node))
}

#[derive(Deserialize, ToSchema, Debug)]
struct BootstrapNodeRequest {
    node_id: String,
}

#[utoipa::path(
    post,
    path = "/bootstrap",
    request_body = BootstrapNodeRequest,
    responses(
        (status = 200, body = ()),
        (status = 500, body = String),
    )
)]
async fn add_bootstrap_node() -> impl IntoResponse {
    return (StatusCode::OK, ());
}
