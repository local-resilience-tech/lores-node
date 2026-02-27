use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    api::helpers::internal_server_error,
    config::config_state::LoresNodeConfigState,
    data::{entities::Region, projections_read::regions::RegionsReadRepo},
    panda_comms::{PandaContainer, RegionId},
    DatabaseState,
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
    Extension(db): Extension<DatabaseState>,
) -> impl IntoResponse {
    let config = config_state.get().await;

    // Get this node id
    let node_id = match panda_container.get_public_key().await {
        Ok(id) => id,
        Err(e) => return internal_server_error(e).into_response(),
    };

    // Get region_ids from config
    let region_ids: Vec<RegionId> = match config.region_ids {
        Some(region_ids) => {
            println!("got region ids {:?}", region_ids);

            region_ids
                .into_iter()
                .map(|id| {
                    let region_id = match RegionId::from_hex(&id) {
                        Ok(id) => id,
                        Err(_) => panic!("Invalid region ID in config: {}", id),
                    };

                    region_id
                })
                .collect()
        }
        None => {
            println!("no region ids found in config");
            return (StatusCode::OK, Json(Vec::<Region>::new())).into_response();
        }
    };

    // Get regions from database
    let repo = RegionsReadRepo::init();
    let db_regions = match repo
        .find_all_for_node(&db.projections_pool, &node_id.to_hex())
        .await
    {
        Ok(regions) => regions,
        Err(e) => return internal_server_error(e).into_response(),
    };

    // Only return regions that are in the config, and if a region is in the config but not in the database, return an unnamed region with just the ID
    let mut result_regions: Vec<Region> = Vec::with_capacity(region_ids.len());
    for id in region_ids {
        let db_region: Option<&Region> = db_regions.iter().find(|r| r.id == id.to_hex());

        let region = match db_region {
            Some(region) => region.clone(),
            None => {
                eprintln!("Region {} not found in database", id);
                Region::unnamed(id, node_id)
            }
        };

        result_regions.push(region);
    }

    (StatusCode::OK, Json(result_regions)).into_response()
}
