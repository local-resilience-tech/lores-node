use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    api::public_api::{client_events::ClientEvent, realtime::RealtimeState},
    local_apps::app_repos::{
        fs::app_repo_from_ref,
        git_app_repos::{checkout_latest_main, clone_git_app_repo},
        AppRepo, AppRepoReference, AppRepoSource,
    },
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create_app_repo))
        .routes(routes!(reload_app_repo))
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

            (StatusCode::CREATED, ())
        }
        Err(e) => {
            eprintln!("Error cloning app repository: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, ())
        }
    }
}

#[utoipa::path(
    get,
    path = "/reload/{repo_name}",
    params(
        ("repo_name" = String, Path),
    ),
    responses(
        (status = 200, body = AppRepo),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    ),
)]
async fn reload_app_repo(Path(repo_name): Path<String>) -> impl IntoResponse {
    let repo_ref = AppRepoReference { repo_name };

    if let Err(e) = checkout_latest_main(&repo_ref) {
        eprintln!("Error checking out latest for app repository: {:?}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response();
    }

    match app_repo_from_ref(&repo_ref) {
        Ok(app_repo) => (StatusCode::OK, Json(app_repo)).into_response(),
        Err(e) => {
            eprintln!("Error retrieving app repository: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response()
        }
    }
}
