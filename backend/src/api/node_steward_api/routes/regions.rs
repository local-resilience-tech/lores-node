use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use p2panda_core::PublicKey;
use serde::Deserialize;
use short_uuid::ShortUuid;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::config_state::LoresNodeConfigState,
    panda_comms::{
        config::{SimplifiedNodeAddress, ThisP2PandaNodeRepo},
        panda_node_container::{build_public_key_from_hex, PandaNodeContainer},
    },
    DatabaseState,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(bootstrap))
        .routes(routes!(create_region))
}

#[derive(Deserialize, ToSchema, Debug)]
pub struct BootstrapNodeData {
    pub network_name: String,
    pub node_id: Option<String>,
}

#[utoipa::path(
    post,
    path = "/bootstrap",
    request_body(content = BootstrapNodeData, content_type = "application/json"),
    responses(
        (status = 200, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = String),
    )
)]
async fn bootstrap(
    Extension(config_state): Extension<LoresNodeConfigState>,
    Extension(panda_container): Extension<PandaNodeContainer>,
    Extension(db): Extension<DatabaseState>,
    axum::extract::Json(data): axum::extract::Json<BootstrapNodeData>,
) -> impl IntoResponse {
    println!("Bootstrapping with data: {:?}", data);
    let repo = ThisP2PandaNodeRepo::init();

    let peer_address: Option<SimplifiedNodeAddress> =
        data.node_id.as_ref().map(|node_id| SimplifiedNodeAddress {
            node_id: node_id.clone(),
        });

    let set_config_result = repo
        .set_network_config(
            &config_state,
            data.network_name.clone(),
            peer_address.clone(),
        )
        .await;
    if let Err(e) = set_config_result {
        eprintln!("Failed to set network config: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to set network config".to_string()),
        )
            .into_response();
    }

    panda_container
        .set_network_name(data.network_name.clone())
        .await;

    let bootstrap_node_id: Option<PublicKey> = match peer_address.clone() {
        Some(bootstrap) => build_public_key_from_hex(bootstrap.node_id.clone()),
        None => None,
    };
    panda_container
        .set_bootstrap_node_id(bootstrap_node_id)
        .await;

    // start the container
    if let Err(e) = panda_container.start(&db.operations_pool).await {
        eprintln!("Failed to start P2PandaContainer: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to start P2PandaContainer".to_string()),
        )
            .into_response();
    }

    (StatusCode::OK, ()).into_response()
}

#[derive(Deserialize, ToSchema, Debug)]
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

    println!("Created new region with ID: {}", region_id);

    return (StatusCode::OK, ()).into_response();
}

fn new_region_id() -> String {
    ShortUuid::generate().to_string()
}

async fn store_new_region_id(config_state: &LoresNodeConfigState) -> Result<String, anyhow::Error> {
    let mut region_id = None;
    config_state
        .update(|config| {
            let mut result = config.clone();
            let mut region_ids = result.region_ids.unwrap_or_else(|| vec![]);

            while region_id.is_none() || region_ids.contains(&region_id.clone().unwrap()) {
                let new_id = new_region_id();
                println!("Trying new region id {}", new_id);
                if !region_ids.contains(&new_id) {
                    region_id = Some(new_id.clone());
                }
            }

            region_ids.push(region_id.clone().unwrap());

            println!("Setting region_ids to {:?}", region_ids);

            result.region_ids = Some(region_ids);
            result
        })
        .await?;

    match region_id {
        Some(id) => Ok(id),
        None => Err(anyhow::anyhow!("Failed to store new region ID")),
    }
}
