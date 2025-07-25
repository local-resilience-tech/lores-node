use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use tracing::event;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    apps::{find_installed_apps, load_app_config},
    panda_comms::{
        container::P2PandaContainer,
        lores_events::{AppRegisteredDataV1, LoResEventPayload},
    },
    projections::entities::LocalApp,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(show_all_local_apps))
        .routes(routes!(register_app))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<LocalApp>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn show_all_local_apps() -> impl IntoResponse {
    let apps = find_installed_apps();
    (StatusCode::OK, Json(apps)).into_response()
}

#[derive(Deserialize, ToSchema, Debug)]
struct AppIdentifier {
    pub name: String,
}

#[utoipa::path(
    post, path = "/register",
    responses(
        (status = 200, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    ),
    request_body(content = AppIdentifier, content_type = "application/json")
)]
async fn register_app(
    Extension(panda_container): Extension<P2PandaContainer>,
    Json(payload): Json<AppIdentifier>,
) -> impl IntoResponse {
    match load_app_config(payload.name.clone()) {
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
            eprintln!("Failed to load app configuration for: {}", payload.name);
            (StatusCode::INTERNAL_SERVER_ERROR, ())
        }
    }
    .into_response()
}
