use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Serialize;
use tracing::event;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    api::{
        auth_api::auth_backend::AuthSession,
        public_api::{client_events::ClientEvent, realtime::RealtimeState},
    },
    data::entities::LocalApp,
    local_apps::{
        app_repos::AppRepoAppReference,
        installed_apps::{
            self,
            fs::{load_local_app_details, InstallAppVersionError},
            AppReference,
        },
    },
    panda_comms::{
        container::P2PandaContainer,
        lores_events::{AppRegisteredDataV1, LoResEventPayload},
    },
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(install_app_definition))
        .routes(routes!(register_app))
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

async fn local_app_updated(app: &LocalApp, realtime_state: &RealtimeState) {
    let client_event = ClientEvent::LocalAppUpdated(app.clone());
    realtime_state.broadcast_app_event(client_event).await;
}
