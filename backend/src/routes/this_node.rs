use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use sqlx::SqlitePool;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    panda_comms::{
        container::P2PandaContainer,
        lores_events::{LoResEventPayload, NodeAnnouncedDataV1},
    },
    repos::{entities::Node, this_node::ThisNodeRepo},
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(show_this_node))
        .routes(routes!(create_this_node))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Option<Node>),
    (status = INTERNAL_SERVER_ERROR, body = String, description = "Internal Server Error"),
))]
async fn show_this_node(Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
    let repo = ThisNodeRepo::init();

    let node = repo.find(&pool).await;

    match node {
        Ok(node) => (StatusCode::OK, Json(node)).into_response(),
        Err(err) => {
            eprintln!("Error fetching node: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

#[derive(Deserialize, ToSchema)]
struct CreateNodeDetails {
    name: String,
}

#[utoipa::path(
    post,
    path = "/create",
    responses(
        (status = 200, body = Node),
        (status = INTERNAL_SERVER_ERROR, body = String, description = "Internal Server Error"),
    ),
    request_body(content = CreateNodeDetails, content_type = "application/json"),
)]
async fn create_this_node(
    Extension(panda_container): Extension<P2PandaContainer>,
    axum::extract::Json(data): axum::extract::Json<CreateNodeDetails>,
) -> impl IntoResponse {
    let event_payload = LoResEventPayload::NodeAnnounced(NodeAnnouncedDataV1 {
        name: data.name.clone(),
    });

    let result = panda_container.publish_persisted(event_payload).await;

    match result {
        Ok(_) => {
            let node = Node {
                id: "1".to_string(),
                name: data.name.clone(),
            };
            (StatusCode::OK, Json(node)).into_response()
        }
        Err(e) => {
            eprintln!("Error publishing event: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}
