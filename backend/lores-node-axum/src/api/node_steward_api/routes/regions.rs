use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    api::auth_api::auth_backend::AuthSession,
    config::config_state::LoresNodeConfigState,
    panda_comms::{
        lores_events::{LoResEventPayload, RegionCreatedDataV1},
        PandaContainer, RegionId,
    },
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create_region))
        .routes(routes!(join_region))
}

#[derive(Deserialize, ToSchema, Debug)]
#[allow(dead_code)]
pub struct CreateRegionData {
    pub slug: String,
    pub name: String,
    pub organisation_name: Option<String>,
    pub organisation_url: Option<String>,
    pub node_steward_conduct_url: Option<String>,
    pub user_conduct_url: Option<String>,
    pub user_privacy_url: Option<String>,
}

#[utoipa::path(
    post,
    path = "/create",
    request_body(content = CreateRegionData, content_type = "application/json"),
    responses(
        (status = 200, body = ()),
        (status = BAD_REQUEST, body = String),
        (status = INTERNAL_SERVER_ERROR, body = String),
    )
)]
async fn create_region(
    Extension(panda_container): Extension<PandaContainer>,
    auth_session: AuthSession,
    Extension(config_state): Extension<LoresNodeConfigState>,
    axum::extract::Json(data): axum::extract::Json<CreateRegionData>,
) -> impl IntoResponse {
    println!("Creating region with data: {:?}", data);

    if data.slug.is_empty() || data.name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json("Slug and name are required".to_string()),
        )
            .into_response();
    }

    println!(
        "Validated region data: slug={}, name={}",
        data.slug, data.name
    );

    // Generate a region ID and store it in the config
    let region_id = match store_new_region_id(&config_state).await {
        Ok(id) => {
            println!("Generated new region ID: {}", id);
            id
        }
        Err(e) => {
            eprintln!("Failed to store new region ID: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to store new region ID".to_string()),
            )
                .into_response();
        }
    };

    // Subscribe to the new region
    let topic_id = match panda_container.join_region(region_id.clone()).await {
        Ok(topic_id) => topic_id,
        Err(e) => {
            eprintln!("Failed to join region: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to join region".to_string()),
            )
                .into_response();
        }
    };

    // Publish the RegionCreated event
    let event_payload = LoResEventPayload::RegionCreated(RegionCreatedDataV1 {
        id: region_id.to_hex(),
        slug: data.slug.clone(),
        name: data.name.clone(),
        organisation_name: data.organisation_name.clone(),
        organisation_url: data.organisation_url.clone(),
        node_steward_conduct_url: data.node_steward_conduct_url.clone(),
        user_conduct_url: data.user_conduct_url.clone(),
        user_privacy_url: data.user_privacy_url.clone(),
    });
    println!("Prepared event payload: {:?}", event_payload);

    if let Err(e) = panda_container
        .publish_persisted(topic_id, event_payload, auth_session.user)
        .await
    {
        eprintln!("Failed to publish RegionCreated event: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to publish RegionCreated event".to_string()),
        )
            .into_response();
    }

    println!("Created new region with ID: {:?}", region_id);

    return (StatusCode::OK, ()).into_response();
}

#[derive(Deserialize, ToSchema, Debug)]
#[allow(dead_code)]
pub struct JoinRegionRequestData {
    pub id: String,
    pub about_your_node: String,
    pub about_your_stewards: String,
    pub node_steward_conduct_url: Option<String>,
}

#[utoipa::path(
    post,
    path = "/join",
    request_body(content = JoinRegionRequestData, content_type = "application/json"),
    responses(
        (status = 200, body = ()),
        (status = BAD_REQUEST, body = String),
        (status = INTERNAL_SERVER_ERROR, body = String),
    )
)]
async fn join_region() -> impl IntoResponse {
    return (StatusCode::INTERNAL_SERVER_ERROR, "not implemented").into_response();
}

async fn store_new_region_id(
    config_state: &LoresNodeConfigState,
) -> Result<RegionId, anyhow::Error> {
    let mut region_id_string: Option<String> = None;
    config_state
        .update(|config| {
            let mut result = config.clone();
            let mut region_ids: Vec<String> = result.region_ids.unwrap_or_else(|| vec![]);

            while region_id_string.is_none()
                || region_ids.contains(&region_id_string.clone().unwrap())
            {
                let new_id_string = RegionId::generate().to_hex();
                println!("Trying new region id {}", new_id_string);
                if !region_ids.contains(&new_id_string) {
                    region_id_string = Some(new_id_string.clone());
                }
            }

            region_ids.push(region_id_string.clone().unwrap());

            println!("Setting region_ids to {:?}", region_ids);

            result.region_ids = Some(region_ids);
            result
        })
        .await?;

    match region_id_string {
        Some(id_string) => match RegionId::from_hex(&id_string) {
            Ok(id) => Ok(id),
            Err(e) => Err(anyhow::anyhow!("Failed to parse new region ID: {:?}", e)),
        },
        None => Err(anyhow::anyhow!("Failed to store new region ID")),
    }
}
