use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use tracing::event;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    api::{
        auth_api::auth_backend::AuthSession,
        public_api::{client_events::ClientEvent, realtime::RealtimeState},
    },
    config::config_state::LoresNodeConfigState,
    data::{entities::LocalApp, projections_read::nodes::NodesReadRepo},
    local_apps::{
        app_repos::{fs::app_repo_from_app_name, AppRepoAppReference},
        installed_apps::{
            self,
            config::{
                load_app_config, load_app_config_schema, save_app_config, validate_app_config,
            },
            fs::{load_local_app_details, InstallAppVersionError},
            AppReference,
        },
        stack_apps::{self},
    },
    panda_comms::{
        container::P2PandaContainer,
        lores_events::{AppRegisteredDataV1, LoResEventPayload},
    },
    DatabaseState,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(install_app_definition))
        .routes(routes!(delete_local_app))
        .routes(routes!(register_app))
        .routes(routes!(deploy_local_app))
        .routes(routes!(remove_deployment_of_local_app))
        .routes(routes!(upgrade_local_app))
        .routes(routes!(get_local_app_config_schema))
        .routes(routes!(get_local_app_config))
        .routes(routes!(update_local_app_config))
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
                _ => InstallLocalAppError::ServerError,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_result)).into_response()
        }
    }
}

#[utoipa::path(
    delete,
    path = "/app/{app_name}",
    params(
        ("app_name" = String, Path),
    ),
    responses(
        (status = OK, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = InstallLocalAppError),
    )
)]
async fn delete_local_app(
    Path(app_name): Path<String>,
    Extension(realtime_state): Extension<RealtimeState>,
) -> impl IntoResponse {
    println!("Deleting local app: {}", app_name);

    let app_ref = AppReference {
        app_name: app_name.clone(),
    };
    let client_event = ClientEvent::LocalAppDeleted(app_ref.clone());
    realtime_state.broadcast_app_event(client_event).await;

    (StatusCode::OK, Json(())).into_response()
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
    Extension(db): Extension<DatabaseState>,
    Extension(config_state): Extension<LoresNodeConfigState>,
    Path(app_name): Path<String>,
) -> impl IntoResponse {
    println!("Deploying local app: {}", app_name);

    let app_ref = AppReference {
        app_name: app_name.clone(),
    };
    let node_repo = NodesReadRepo::init();
    let config = config_state.get().await;
    let public_key_hex = match config.public_key_hex {
        Some(key) => key,
        None => {
            eprintln!("No public key hex found in config");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Public key not found").into_response();
        }
    };

    let node = match node_repo
        .find(&db.projections_pool, public_key_hex.clone())
        .await
    {
        Ok(Some(node)) => node,
        Ok(None) => {
            eprintln!("Node not found for public key: {}", public_key_hex);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Node not found").into_response();
        }
        Err(e) => {
            eprintln!("Failed to find node: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response();
        }
    };

    let result = stack_apps::deploy_local_app(&app_ref, &node);

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
    Extension(db): Extension<DatabaseState>,
    Extension(config_state): Extension<LoresNodeConfigState>,
    Extension(realtime_state): Extension<RealtimeState>,
    Path(app_name): Path<String>,
) -> impl IntoResponse {
    println!("Removing deployment of local app: {}", app_name);

    let config = config_state.get().await;
    let public_key_hex = match config.public_key_hex {
        Some(key) => key,
        None => {
            eprintln!("No public key hex found in config");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Public key not found").into_response();
        }
    };

    let node_repo = NodesReadRepo::init();
    let node = match node_repo
        .find(&db.projections_pool, public_key_hex.clone())
        .await
    {
        Ok(Some(node)) => node,
        Ok(None) => {
            eprintln!("Node not found for public key: {}", public_key_hex);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Node not found").into_response();
        }
        Err(e) => {
            eprintln!("Failed to find node: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response();
        }
    };

    let app_ref = AppReference {
        app_name: app_name.clone(),
    };
    let result = stack_apps::remove_local_app(&app_ref, &node);

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
    auth_session: AuthSession,
    Json(payload): Json<AppReference>,
) -> impl IntoResponse {
    match load_local_app_details(&payload) {
        Some(app) => {
            let event_payload = LoResEventPayload::AppRegistered(AppRegisteredDataV1 {
                name: app.name.clone(),
                version: app.version.clone(),
            });
            let publish_result = panda_container
                .publish_persisted(event_payload, auth_session.user)
                .await;
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

#[utoipa::path(
    get,
    path = "/app/{app_name}/config_schema",
    params(
        ("app_name" = String, Path),
    ),
    responses(
        (status = 200, body = serde_json::Value),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    )
)]
async fn get_local_app_config_schema(Path(app_name): Path<String>) -> impl IntoResponse {
    println!("Fetching config schema for local app: {}", app_name);
    let app_ref = AppReference { app_name };

    match load_app_config_schema(&app_ref) {
        Ok(Some(schema)) => (StatusCode::OK, Json(schema)).into_response(),
        Ok(None) => {
            eprintln!("No config schema found for app: {}", app_ref.app_name);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "No config schema found"})),
            )
                .into_response()
        }
        Err(e) => {
            eprintln!(
                "Error loading config schema for app '{}': {}",
                app_ref.app_name, e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to load config schema"})),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/app/{app_name}/config",
    params(
        ("app_name" = String, Path),
    ),
    responses(
        (status = 200, body = serde_json::Value),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    )
)]
async fn get_local_app_config(Path(app_name): Path<String>) -> impl IntoResponse {
    println!("Fetching config for local app: {}", app_name);
    let app_ref = AppReference { app_name };

    match load_app_config(&app_ref) {
        Ok(config) => (StatusCode::OK, Json(config)).into_response(),
        Err(e) => {
            eprintln!("Error loading config for app '{}': {}", app_ref.app_name, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to load config"})),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    put,
    path = "/app/{app_name}/config",
    params(
        ("app_name" = String, Path),
    ),
    request_body(content_type = "application/json"),
    responses(
        (status = 200, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    )
)]
async fn update_local_app_config(
    Path(app_name): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    println!("Updating local app config with payload: {:?}", payload);
    let app_ref = AppReference { app_name };

    match validate_app_config(&app_ref, &payload) {
        Ok(_) => println!("Config validated successfully"),
        Err(e) => {
            eprintln!("Config validation failed: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Config validation failed"})),
            )
                .into_response();
        }
    }

    match save_app_config(&app_ref, &payload) {
        Ok(_) => (StatusCode::OK, Json(())).into_response(),
        Err(e) => {
            eprintln!("Error saving config for app '{}': {}", app_ref.app_name, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to save config"})),
            )
                .into_response()
        }
    }
}

async fn local_app_updated(app: &LocalApp, realtime_state: &RealtimeState) {
    let client_event = ClientEvent::LocalAppUpdated(app.clone());
    realtime_state.broadcast_app_event(client_event).await;
}
