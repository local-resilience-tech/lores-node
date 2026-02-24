use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    api::{
        auth_api::auth_backend::AuthSession,
        public_api::{client_events::ClientEvent, realtime::RealtimeState},
    },
    config::config_state::LoresNodeConfigState,
    data::entities::Region,
    panda_comms::{
        lores_events::{LoResEventPayload, RegionCreatedDataV1},
        PandaContainer, RegionId,
    },
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        // .routes(routes!(bootstrap))
        .routes(routes!(create_region))
}

// #[derive(Deserialize, ToSchema, Debug)]
// pub struct BootstrapNodeData {
//     pub network_name: String,
//     pub node_id: Option<String>,
// }

// #[utoipa::path(
//     post,
//     path = "/bootstrap",
//     request_body(content = BootstrapNodeData, content_type = "application/json"),
//     responses(
//         (status = 200, body = ()),
//         (status = INTERNAL_SERVER_ERROR, body = String),
//     )
// )]
// async fn bootstrap(
//     Extension(config_state): Extension<LoresNodeConfigState>,
//     Extension(panda_container): Extension<PandaContainer>,
//     Extension(db): Extension<DatabaseState>,
//     axum::extract::Json(data): axum::extract::Json<BootstrapNodeData>,
// ) -> impl IntoResponse {
//     println!("Bootstrapping with data: {:?}", data);
//     let repo = ThisP2PandaNodeRepo::init();

//     let peer_address: Option<SimplifiedNodeAddress> =
//         data.node_id.as_ref().map(|node_id| SimplifiedNodeAddress {
//             node_id: node_id.clone(),
//         });

//     let set_config_result = repo
//         .set_network_config(
//             &config_state,
//             data.network_name.clone(),
//             peer_address.clone(),
//         )
//         .await;
//     if let Err(e) = set_config_result {
//         eprintln!("Failed to set network config: {:?}", e);
//         return (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             Json("Failed to set network config".to_string()),
//         )
//             .into_response();
//     }

//     panda_container
//         .set_network_name(data.network_name.clone())
//         .await;

//     let bootstrap_node_id: Option<PublicKey> = match peer_address.clone() {
//         Some(bootstrap) => build_public_key_from_hex(bootstrap.node_id.clone()),
//         None => None,
//     };
//     panda_container
//         .set_bootstrap_node_id(bootstrap_node_id)
//         .await;

//     // start the container
//     if let Err(e) = panda_container.start(&db.operations_pool).await {
//         eprintln!("Failed to start P2PandaContainer: {:?}", e);
//         return (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             Json("Failed to start P2PandaContainer".to_string()),
//         )
//             .into_response();
//     }

//     (StatusCode::OK, ()).into_response()
// }

#[derive(Deserialize, ToSchema, Debug)]
#[allow(dead_code)]
pub struct CreateRegionData {
    pub slug: String,
    pub name: String,
    pub organisation_name: Option<String>,
    pub url: Option<String>,
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
    Extension(realtime_state): Extension<RealtimeState>,
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
        Ok(id) => id,
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
        slug: data.slug.clone(),
        name: data.name.clone(),
        organisation_name: data.organisation_name.clone(),
        url: data.url.clone(),
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

    realtime_state
        .broadcast_app_event(ClientEvent::JoinedRegion(Region::unnamed(
            region_id.to_string(),
        )))
        .await;

    return (StatusCode::OK, ()).into_response();
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
