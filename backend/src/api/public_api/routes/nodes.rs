use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    projections::{entities::NodeDetails, projections_read::nodes::NodesReadRepo},
    DatabaseState,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_nodes))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<NodeDetails>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_nodes(Extension(db): Extension<DatabaseState>) -> impl IntoResponse {
    let repo = NodesReadRepo::init();

    repo.all(&db.projections_pool)
        .await
        .map(|nodes| (StatusCode::OK, Json(nodes)))
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(())))
        .into_response()
}
