use axum::{http::StatusCode, response::IntoResponse};
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(show_region))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = ()),
    (status = NOT_FOUND, body = ()),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn show_region() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, ()).into_response()
}
