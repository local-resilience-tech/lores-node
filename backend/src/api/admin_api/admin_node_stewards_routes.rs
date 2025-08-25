use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{data::node_data::node_stewards::NodeStewardsRepo, DatabaseState};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_node_stewards))
}

#[derive(Serialize, ToSchema)]
pub struct NodeSteward {
    pub id: String,
    pub name: String,
    pub active: bool,
}

#[utoipa::path(get, path = "/",
    responses(
        (status = OK, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    ),
)]
async fn list_node_stewards(Extension(db): Extension<DatabaseState>) -> impl IntoResponse {
    let repo = NodeStewardsRepo::init();

    match repo.all(&db.node_data_pool).await {
        Ok(nodes) => {
            let result_nodes: Vec<NodeSteward> = nodes
                .into_iter()
                .map(|node| NodeSteward {
                    id: node.id,
                    name: node.name,
                    active: node.active,
                })
                .collect();
            (StatusCode::OK, Json(result_nodes)).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response(),
    }
}
