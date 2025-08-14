use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::event;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    admin_api::{client_events::ClientEvent, realtime::RealtimeState},
    local_apps::{
        app_repos::{
            fs::app_repo_from_app_name,
            git_app_repos::{with_checked_out_app_version, CheckoutAppVersionError},
            AppRepoAppReference,
        },
        installed_apps::{
            self,
            fs::{load_app_config, InstallAppVersionError},
            AppReference,
        },
        stack_apps::{self, find_deployed_local_apps},
    },
    panda_comms::{
        container::P2PandaContainer,
        lores_events::{AppRegisteredDataV1, LoResEventPayload},
    },
    projections::entities::LocalApp,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_local_apps))
        .routes(routes!(install_app_definition))
        .routes(routes!(register_app))
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

#[derive(Serialize, ToSchema, Debug)]
enum InstallLocalAppError {
    InUse,
    ServerError,
}

#[utoipa::path(
    post,
    path = "/definitions",
    responses(
        (status = CREATED, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = InstallLocalAppError),
    ),
    request_body(content = AppRepoAppReference, content_type = "application/json")
)]
async fn install_app_definition(
    Extension(realtime_state): Extension<RealtimeState>,
    Json(payload): Json<AppRepoAppReference>,
) -> impl IntoResponse {
    let source = payload.clone();
    let target = AppReference {
        app_name: payload.app_name.clone(),
    };

    println!(
        "Installing app definition from repo '{}' for app '{} {:?}'",
        source.repo_name, source.app_name, source.version
    );

    let result = installed_apps::fs::install_app_definition(&source, &target);

    match result {
        Ok(app) => {
            local_app_updated(&app, &realtime_state).await;
            println!("App definition installed: {}", app.name);

            (StatusCode::CREATED, Json(())).into_response()
        }
        Err(e) => {
            println!("Error installing app definition: {:?}", e);

            let error_result = match e {
                InstallAppVersionError::InUse => InstallLocalAppError::InUse,
                InstallAppVersionError::FileSystemError => InstallLocalAppError::ServerError,
                InstallAppVersionError::LoadingAppError => InstallLocalAppError::ServerError,
                InstallAppVersionError::CheckoutError(_) => InstallLocalAppError::ServerError,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_result)).into_response()
        }
    }
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
    Path(app_name): Path<String>,
    Json(payload): Json<LocalAppUpgradeParams>,
) -> impl IntoResponse {
    println!(
        "Deploying local app {} to version {}",
        app_name, payload.target_version
    );

    let app_ref = match app_repo_from_app_name(&app_name) {
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
        repo_name: app_ref.repo_name.clone(),
        app_name: app_name.clone(),
        version: payload.target_version.clone(),
    };

    let result = with_checked_out_app_version(&repo_app_ref);

    match result {
        Ok(_) => {
            println!("Successfully upgraded local app: {}", app_name);
            (StatusCode::OK, ()).into_response()
        }
        Err(CheckoutAppVersionError::InUse) => {
            eprintln!(
                "Local app '{}' is currently in use and cannot be upgraded.",
                app_name
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UpgradeLocalAppError::InUse),
            )
                .into_response()
        }
        Err(CheckoutAppVersionError::Other(e)) => {
            eprintln!("Failed to upgrade local app '{}': {:?}", app_name, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UpgradeLocalAppError::ServerError),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    post, path = "/register",
    request_body(content = AppReference, content_type = "application/json"),
    responses(
        (status = 200, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    )
)]
async fn register_app(
    Extension(panda_container): Extension<P2PandaContainer>,
    Json(payload): Json<AppReference>,
) -> impl IntoResponse {
    match load_app_config(&payload) {
        Some(app) => {
            let event_payload = LoResEventPayload::AppRegistered(AppRegisteredDataV1 {
                name: app.name.clone(),
                version: app.version.clone(),
            });
            let publish_result = panda_container.publish_persisted(event_payload).await;
            match publish_result {
                Ok(_) => {
                    event!(tracing::Level::INFO, "App registered: {}", app.name);
                    (StatusCode::OK, ())
                }
                Err(e) => {
                    eprintln!("Failed to publish app registration event: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, ())
                }
            }
        }
        None => {
            eprintln!("Failed to load app configuration for: {}", payload.app_name);
            (StatusCode::INTERNAL_SERVER_ERROR, ())
        }
    }
    .into_response()
}

async fn local_app_updated(app: &LocalApp, realtime_state: &RealtimeState) {
    let client_event = ClientEvent::LocalAppUpdated(app.clone());
    realtime_state.broadcast_app_event(client_event).await;
}
