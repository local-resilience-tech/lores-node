use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use lores_p2panda::operations::LogType;
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    api::{
        auth_api::auth_backend::AuthSession,
        helpers::{bad_request, internal_server_error},
    },
    data::entities::LocalApp,
    panda_comms::{
        lores_events::{AppRegisteredDataV1, LoResEventPayload},
        PandaContainer, RegionId,
    },
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(register_app))
}

#[derive(Deserialize, ToSchema, Debug, Clone)]
pub struct AppRegionReference {
    pub region_id: String,
    pub app: LocalApp,
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
    let topic_id = PandaContainer::get_region_topic_id(&region_id);
    if let Err(e) = panda_container
        .publish_persisted(topic_id, LogType::Admin, event_payload, auth_session.user)
        .await
    {
        return internal_server_error(e).into_response();
    }

    (StatusCode::OK, ()).into_response()
}
