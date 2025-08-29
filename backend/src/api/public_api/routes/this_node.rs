use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::config_state::LoresNodeConfigState,
    data::{entities::Node, projections_read::nodes::NodesReadRepo},
    panda_comms::{
        container::P2PandaContainer,
        lores_events::{LoResEventPayload, NodeAnnouncedDataV1},
    },
    DatabaseState,
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
pub async fn show_this_node(
    Extension(db): Extension<DatabaseState>,
    Extension(config_state): Extension<LoresNodeConfigState>,
) -> impl IntoResponse {
    let repo = NodesReadRepo::init();
    let config = config_state.get().await;

    match config.public_key_hex {
        Some(public_key_hex) => {
            let node = repo.find(&db.projections_pool, public_key_hex).await;

            match node {
                Ok(node) => (StatusCode::OK, Json(node)).into_response(),
                Err(err) => {
                    eprintln!("Error fetching node: {}", err);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
                }
            }
        }
        None => {
            eprintln!("No public key hex found in config");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Public key not found").into_response();
        }
    }
}

#[derive(Deserialize, ToSchema)]
struct CreateNodeDetails {
    name: String,
}

#[utoipa::path(
    post,
    path = "/",
    responses(
        (status = CREATED, body = Node),
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
            (StatusCode::CREATED, Json(node)).into_response()
        }
        Err(e) => {
            eprintln!("Error publishing event: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}
