use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    DatabaseState,
    api::{
        auth_api::auth_backend::AuthSession,
        helpers::{bad_request, internal_server_error},
    },
    data::{
        entities::{LocalApp, LocalAppSource},
        node_data::local_apps_repo::LocalAppsRepo,
    },
    panda_comms::{
        PandaContainer, RegionAdminTopic, RegionId,
        lores_events::{AppRegisteredDataV1, LoResEventPayload},
    },
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(register_app))
        .routes(routes!(create_local_app))
}

#[derive(Deserialize, ToSchema, Debug, Clone)]
pub struct AppRegionReference {
    pub region_id: String,
    pub app: LocalApp,
}

#[derive(Deserialize, ToSchema, Debug, Clone)]
pub struct LocalAppFormData {
    pub name: String,
    pub version: String,
    pub instance_id: Option<String>,
}

impl LocalAppFormData {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("name must not be empty".to_string());
        }
        if self.version.trim().is_empty() {
            return Err("version must not be empty".to_string());
        }

        Ok(())
    }

    pub fn to_local_app(&self) -> LocalApp {
        LocalApp {
            name: self.name.clone(),
            version: self.version.clone(),
            url: None,
            source: LocalAppSource::Db,
            instance_id: self.instance_id.clone(),
        }
    }
}

#[utoipa::path(
    post, path = "/register",
    request_body(content = AppRegionReference, content_type = "application/json"),
    responses(
        (status = 200, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    )
)]
async fn register_app(
    Extension(panda_container): Extension<PandaContainer>,
    auth_session: AuthSession,
    Json(payload): Json<AppRegionReference>,
) -> impl IntoResponse {
    // Get region_id from data and validate it
    let region_id = match RegionId::from_hex(&payload.region_id) {
        Ok(id) => id,
        Err(e) => return bad_request(e).into_response(),
    };

    let event_payload = LoResEventPayload::AppRegistered(AppRegisteredDataV1 {
        name: payload.app.name.clone(),
        version: payload.app.version.clone(),
    });
    println!("Prepared event payload: {:?}", event_payload);

    // Publish the operation
    if let Err(e) = panda_container
        .publish_persisted(
            &RegionAdminTopic::new(region_id),
            event_payload,
            auth_session.user,
        )
        .await
    {
        return internal_server_error(e).into_response();
    }

    (StatusCode::OK, ()).into_response()
}

#[utoipa::path(
    post, path = "/create",
    request_body(content = LocalAppFormData, content_type = "application/json"),
    responses(
        (status = 201, body = LocalApp),
        (status = BAD_REQUEST, body = String),
        (status = INTERNAL_SERVER_ERROR, body = String),
    )
)]
async fn create_local_app(
    _auth_session: AuthSession,
    Extension(db): Extension<DatabaseState>,
    Json(payload): Json<LocalAppFormData>,
) -> impl IntoResponse {
    if let Err(error) = payload.validate() {
        return bad_request(error).into_response();
    }

    let app = payload.to_local_app();

    let created = match LocalAppsRepo::init().create(&db.node_data_pool, &app).await {
        Ok(app) => app,
        Err(sqlx::Error::Database(e)) => {
            // Treat uniqueness and similar DB constraint errors as bad request.
            return bad_request(e).into_response();
        }
        Err(e) => return internal_server_error(e).into_response(),
    };

    (StatusCode::CREATED, Json(created)).into_response()
}
