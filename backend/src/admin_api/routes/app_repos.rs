use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(register_app_repo))
}

#[derive(Deserialize, ToSchema)]
struct AppRepo {
    pub git_url: String,
}

#[utoipa::path(
    post,
    path = "/",
    responses(
        (status = 201, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    ),
    request_body(content = AppRepo, content_type = "application/json")
)]
async fn register_app_repo(Json(payload): Json<AppRepo>) -> impl IntoResponse {
    println!("Registering app repository: {}", payload.git_url);

    // Logic to handle app repository registration
    (StatusCode::CREATED, Json(()))
}
