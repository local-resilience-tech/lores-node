use axum::{http::StatusCode, response::IntoResponse, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::data::entities::Region;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(show_region))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Option<Region>, description = "Returns the current region's network ID if available"),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn show_region(// Extension(config_state): Extension<LoresNodeConfigState>,
) -> impl IntoResponse {
    // match config.network_name {
    //     Some(network_id) => {
    //         println!("got network id {}", network_id);
    //         (StatusCode::OK, Json(Some(Region { network_id })))
    //     }
    //     None => {
    //         println!("no network id");
    //         (StatusCode::OK, Json(None))
    //     }
    // }
    // .into_response()

    (StatusCode::OK, Json(None::<Region>)).into_response()
}
