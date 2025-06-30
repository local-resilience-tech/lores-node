use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::SqlitePool;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::repos::{
    entities::{NodeDetails, Region},
    nodes::NodesRepo,
    this_p2panda_node::ThisP2PandaNodeRepo,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(show_region))
        .routes(routes!(nodes))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Option<Region>, description = "Returns the current region's network ID if available"),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn show_region(Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
    let repo = ThisP2PandaNodeRepo::init();

    repo.get_network_name(&pool)
        .await
        .map(|network_id| match network_id {
            Some(network_id) => {
                println!("got network id {}", network_id);
                (StatusCode::OK, Json(Some(Region { network_id })))
            }
            None => {
                println!("no network id");
                (StatusCode::OK, Json(None))
            }
        })
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(())))
        .into_response()
}

#[utoipa::path(get, path = "/nodes", responses(
    (status = 200, body = Vec<NodeDetails>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn nodes(Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
    let repo = NodesRepo::init();

    repo.all(&pool)
        .await
        .map(|nodes| (StatusCode::OK, Json(nodes)))
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(())))
        .into_response()
}
