use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{config::config_state::LoresNodeConfigState, data::entities::Region};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(show_region))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Option<Region>, description = "Returns the current region if available"),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn show_region(
    Extension(config_state): Extension<LoresNodeConfigState>,
) -> impl IntoResponse {
    let config = config_state.get().await;

    match config.region_name {
        Some(region_name) => {
            println!("got region name {}", region_name);
            (StatusCode::OK, Json(Some(Region { name: region_name })))
        }
        None => {
            println!("no region name found in config");
            (StatusCode::OK, Json(None))
        }
    }
    .into_response()
}
