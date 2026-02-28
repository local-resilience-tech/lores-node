use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    data::{entities::RegionNodeDetails, projections_read::region_nodes::RegionNodesReadRepo},
    DatabaseState,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_nodes))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<RegionNodeDetails>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_nodes(Extension(db): Extension<DatabaseState>) -> impl IntoResponse {
    let repo = RegionNodesReadRepo::init();

    let region_id = "dummy_region_id".to_string();
    repo.find_all_detailed(&db.projections_pool, &region_id)
        .await
        .map(|nodes| (StatusCode::OK, Json(nodes)))
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(())))
        .into_response()
}
