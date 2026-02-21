use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{config::config_state::LoresNodeConfigState, data::entities::Region};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_regions))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<Region>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_regions(
    Extension(config_state): Extension<LoresNodeConfigState>,
) -> impl IntoResponse {
    let config = config_state.get().await;

    match config.region_ids {
        Some(region_ids) => {
            println!("got region ids {:?}", region_ids);

            let regions: Vec<Region> = region_ids
                .into_iter()
                .map(|id| Region {
                    id: id.clone(),
                    name: "unknown".to_string(),
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
