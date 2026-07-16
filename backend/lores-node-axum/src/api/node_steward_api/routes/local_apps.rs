use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use tracing::warn;
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    DatabaseState,
    api::{
        auth_api::auth_backend::AuthSession,
        helpers::{bad_request, internal_server_error},
        public_api::{client_events::ClientEvent, realtime::RealtimeState},
    },
    data::{
        entities::{LocalApp, LocalAppSource},
        node_data::local_apps_repo::LocalAppsRepo,
        projections_read::regions::RegionsReadRepo,
    },
    local_apps::find_local_app,
    panda_comms::{
        PandaContainer, RegionAdminTopic, RegionId,
        lores_events::{AppRegisteredDataV1, LoResEventPayload},
    },
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(register_app))
        .routes(routes!(create_local_app))
        .routes(routes!(update_local_app))
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
            bound_to_region_id: None,
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
    Extension(db): Extension<DatabaseState>,
    Extension(realtime_state): Extension<RealtimeState>,
    auth_session: AuthSession,
    Json(payload): Json<AppRegionReference>,
) -> impl IntoResponse {
    // Get region_id from data and validate it
    let region_id = match RegionId::from_hex(&payload.region_id) {
        Ok(id) => id,
        Err(e) => return bad_request(e).into_response(),
    };

    // Verify the region is known to this node
    match RegionsReadRepo::init()
        .find(&db.projections_pool, &payload.region_id)
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => return bad_request("Region not found").into_response(),
        Err(e) => return internal_server_error(e).into_response(),
    }

    // Verify the app exists locally (Docker or DB)
    let existing_app = match find_local_app(&db.node_data_pool, &payload.app.name, &payload.app.instance_id).await {
        Ok(Some(app)) => app,
        Ok(None) => return bad_request("App not found").into_response(),
        Err(e) => return internal_server_error(e).into_response(),
    };

    // Check if already bound to a region
    let already_bound = match &existing_app.bound_to_region_id {
        Some(bound_id) if bound_id == &payload.region_id => true,
        Some(_) => return bad_request("App is already bound to a different region").into_response(),
        None => false,
    };

    // Persist the binding in node_data before announcing to the network
    let binding_event = if !already_bound {
        if let Err(e) = LocalAppsRepo::init()
            .bind_to_region(
                &db.node_data_pool,
                &payload.app.name,
                &payload.app.instance_id,
                &payload.region_id,
            )
            .await
        {
            return internal_server_error(e).into_response();
        }

        let mut updated_app = existing_app;
        updated_app.bound_to_region_id = Some(payload.region_id.clone());
        Some(ClientEvent::LocalAppUpdated(updated_app))
    } else {
        None
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

    if let Some(event) = binding_event {
        realtime_state.broadcast_app_event(event).await;
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
    Extension(realtime_state): Extension<RealtimeState>,
    Json(payload): Json<LocalAppFormData>,
) -> impl IntoResponse {
    if let Err(error) = payload.validate() {
        return bad_request(error).into_response();
    }

    let app = payload.to_local_app();

    let created = match LocalAppsRepo::init().create(&db.node_data_pool, &app).await {
        Ok(app) => app,
        Err(sqlx::Error::Database(e)) => {
            warn!("Database error creating local app: {e}");
            if e.code().as_deref() == Some("2067") {
                return bad_request("Name and instance ID must be unique").into_response();
            }
            return internal_server_error("A database error occurred").into_response();
        }
        Err(e) => return internal_server_error(e).into_response(),
    };

    realtime_state
        .broadcast_app_event(ClientEvent::LocalAppCreated(created.clone()))
        .await;

    (StatusCode::CREATED, Json(created)).into_response()
}

#[utoipa::path(
    put, path = "/update",
    request_body(content = LocalAppFormData, content_type = "application/json"),
    responses(
        (status = 200, body = LocalApp),
        (status = BAD_REQUEST, body = String),
        (status = NOT_FOUND, body = String),
        (status = INTERNAL_SERVER_ERROR, body = String),
    )
)]
async fn update_local_app(
    _auth_session: AuthSession,
    Extension(db): Extension<DatabaseState>,
    Extension(realtime_state): Extension<RealtimeState>,
    Json(payload): Json<LocalAppFormData>,
) -> impl IntoResponse {
    if let Err(error) = payload.validate() {
        return bad_request(error).into_response();
    }

    let app = payload.to_local_app();

    let updated = match LocalAppsRepo::init().update(&db.node_data_pool, &app).await {
        Ok(Some(app)) => app,
        Ok(None) => return (StatusCode::NOT_FOUND, Json("App not found")).into_response(),
        Err(e) => return internal_server_error(e).into_response(),
    };

    realtime_state
        .broadcast_app_event(ClientEvent::LocalAppUpdated(updated.clone()))
        .await;

    (StatusCode::OK, Json(updated)).into_response()
}
