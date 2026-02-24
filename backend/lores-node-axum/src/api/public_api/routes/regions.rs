use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::config_state::LoresNodeConfigState,
    data::entities::Region,
    panda_comms::{PandaContainer, RegionId},
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_regions))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<Region>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_regions(
    Extension(panda_container): Extension<PandaContainer>,
    Extension(config_state): Extension<LoresNodeConfigState>,
) -> impl IntoResponse {
    let config = config_state.get().await;

    // Get this node id
    let node_id = match panda_container.get_public_key().await {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to get node ID: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to get node ID".to_string()),
            )
                .into_response();
        }
    };

    match config.region_ids {
        Some(region_ids) => {
            println!("got region ids {:?}", region_ids);

            let regions: Vec<Region> = region_ids
                .into_iter()
                .map(|id| {
                    let region_id = match RegionId::from_hex(&id) {
                        Ok(id) => id,
                        Err(_) => panic!("Invalid region ID in config: {}", id),
                    };

                    Region::unnamed(region_id, node_id.clone())
                })
                .collect();
            (StatusCode::OK, Json(regions))
        }
        None => {
            println!("no region ids found in config");
            (StatusCode::OK, Json(Vec::new()))
        }
    }
    .into_response()
}
