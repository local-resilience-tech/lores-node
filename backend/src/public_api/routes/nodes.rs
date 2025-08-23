use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::SqlitePool;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::projections::{entities::NodeDetails, projections_read::nodes::NodesReadRepo};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_nodes))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<NodeDetails>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_nodes(Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
    let repo = NodesReadRepo::init();

    repo.all(&pool)
        .await
        .map(|nodes| (StatusCode::OK, Json(nodes)))
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(())))
        .into_response()
}
