use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    data::entities::LocalApp,
    local_apps::{
        app_repos::{fs::app_repo_from_app_name, AppRepoAppReference},
        installed_apps::{self, fs::InstallAppVersionError, AppReference},
        stack_apps::{self, find_deployed_local_apps},
    },
};

use super::super::{client_events::ClientEvent, realtime::RealtimeState};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_local_apps))
        .routes(routes!(deploy_local_app))
        .routes(routes!(remove_deployment_of_local_app))
        .routes(routes!(upgrade_local_app))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<LocalApp>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_local_apps() -> impl IntoResponse {
    let apps = find_deployed_local_apps();
    (StatusCode::OK, Json(apps)).into_response()
}

#[utoipa::path(
    post,
    path = "/app/{app_name}/deploy",
    params(
        ("app_name" = String, Path),
    ),
    responses(
        (status = OK, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = String),
    ),
)]
async fn deploy_local_app(
    Extension(realtime_state): Extension<RealtimeState>,
    Path(app_name): Path<String>,
) -> impl IntoResponse {
    println!("Deploying local app: {}", app_name);

    let app_ref = AppReference {
        app_name: app_name.clone(),
    };
    let result = stack_apps::deploy_local_app(&app_ref);

    match result {
        Ok(app) => {
            local_app_updated(&app, &realtime_state).await;

            println!("Successfully deployed local app: {}", app_name);
            (StatusCode::OK, Json(())).into_response()
        }
        Err(e) => {
            eprintln!("Failed to deploy local app '{}': {}", app_name, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response()
        }
    }
}

#[utoipa::path(
    delete,
    path = "/app/{app_name}/deploy",
    params(
        ("app_name" = String, Path),
    ),
    responses(
        (status = OK, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = String),
    ),
)]
async fn remove_deployment_of_local_app(
    Extension(realtime_state): Extension<RealtimeState>,
    Path(app_name): Path<String>,
) -> impl IntoResponse {
    println!("Removing deployment of local app: {}", app_name);

    let app_ref = AppReference {
        app_name: app_name.clone(),
    };
    let result = stack_apps::remove_local_app(&app_ref);

    match result {
        Ok(app) => {
            local_app_updated(&app, &realtime_state).await;

            println!("Successfully removed local app: {}", app_name);
            (StatusCode::OK, Json(())).into_response()
        }
        Err(e) => {
            eprintln!(
                "Failed to remove deployment of local app '{}': {}",
                app_name, e
            );
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response()
        }
    }
}

#[derive(Deserialize, ToSchema, Debug)]
struct LocalAppUpgradeParams {
    target_version: String,
}

#[derive(Serialize, ToSchema, Debug)]
enum UpgradeLocalAppError {
    AppNotFound,
    InUse,
    ServerError,
}

#[utoipa::path(
    post,
    path = "/app/{app_name}/upgrade",
    params(
        ("app_name" = String, Path),
    ),
    request_body(content = LocalAppUpgradeParams, content_type = "application/json"),
    responses(
        (status = OK, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = UpgradeLocalAppError),
    ),
)]
async fn upgrade_local_app(
    Extension(realtime_state): Extension<RealtimeState>,
    Path(app_name): Path<String>,
    Json(payload): Json<LocalAppUpgradeParams>,
) -> impl IntoResponse {
    println!(
        "Deploying local app {} to version {}",
        app_name, payload.target_version
    );

    let repo_ref = match app_repo_from_app_name(&app_name) {
        Some(app_ref) => app_ref,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UpgradeLocalAppError::AppNotFound),
            )
                .into_response()
        }
    };

    let repo_app_ref = AppRepoAppReference {
        repo_name: repo_ref.repo_name.clone(),
        app_name: app_name.clone(),
        version: payload.target_version.clone(),
    };

    let app_ref = AppReference {
        app_name: app_name.clone(),
    };

    let result = installed_apps::fs::install_app_definition(&repo_app_ref, &app_ref);

    match result {
        Ok(app) => {
            local_app_updated(&app, &realtime_state).await;
            println!("App definition installed: {}", app.name);

            (StatusCode::CREATED, Json(())).into_response()
        }
        Err(e) => {
            println!("Error installing app definition: {:?}", e);

            let error_result = match e {
                InstallAppVersionError::InUse => UpgradeLocalAppError::InUse,
                _ => UpgradeLocalAppError::ServerError,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_result)).into_response()
        }
    }
}

async fn local_app_updated(app: &LocalApp, realtime_state: &RealtimeState) {
    let client_event = ClientEvent::LocalAppUpdated(app.clone());
    realtime_state.broadcast_app_event(client_event).await;
}
