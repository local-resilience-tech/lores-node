use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    admin_api::{client_events::ClientEvent, realtime::RealtimeState},
    local_apps::app_repos::{self, git::clone_git_app_repo, AppRepo, AppRepoSource},
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create_app_repo))
        .routes(routes!(list_app_repos))
}

#[utoipa::path(
    post,
    path = "/",
    responses(
        (status = 201, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    ),
    request_body(content = AppRepoSource, content_type = "application/json")
)]
async fn create_app_repo(
    Extension(realtime_state): Extension<RealtimeState>,
    Json(payload): Json<AppRepoSource>,
) -> impl IntoResponse {
    println!("Registering app repository: {}", payload.git_url);

    let result = clone_git_app_repo(&payload).await;

    match result {
        Ok(app_repo) => {
            let event = ClientEvent::AppRepoUpdated(app_repo);
            realtime_state.broadcast_app_event(event).await;

            (StatusCode::CREATED, Json(()))
        }
        Err(e) => {
            eprintln!("Error cloning app repository: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(()))
        }
    }
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, body = [AppRepo]),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    ),
)]
async fn list_app_repos() -> impl IntoResponse {
    let repos = app_repos::installed::list_installed_app_repos();

    (StatusCode::OK, Json(repos))
}
