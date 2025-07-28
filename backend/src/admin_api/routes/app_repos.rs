use axum::{http::StatusCode, response::IntoResponse, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::app_repos::{git::clone_git_app_repo, AppRepo};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(create_app_repo))
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
async fn create_app_repo(Json(payload): Json<AppRepo>) -> impl IntoResponse {
    println!("Registering app repository: {}", payload.git_url);

    let result = clone_git_app_repo(&payload).await;

    match result {
        Ok(_) => (StatusCode::CREATED, Json(())),
        Err(e) => {
            eprintln!("Error cloning app repository: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(()))
        }
    }
}
