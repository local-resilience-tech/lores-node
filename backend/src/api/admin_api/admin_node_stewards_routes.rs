use axum::{http::StatusCode, response::IntoResponse};
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_node_stewards))
}

#[utoipa::path(get, path = "/",
    responses(
        (status = OK, body = ()),
    ),
)]
async fn list_node_stewards() -> impl IntoResponse {
    (StatusCode::OK, ())
}
