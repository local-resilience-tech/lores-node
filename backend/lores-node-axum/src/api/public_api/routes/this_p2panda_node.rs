use tracing::info;
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::panda_comms::PandaContainer;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(show_this_panda_node))
        .routes(routes!(p2panda_log_counts))
        .routes(routes!(node_status))
}

#[derive(sqlx::FromRow, Serialize, ToSchema)]
pub struct P2PandaNodeDetails {
    pub panda_node_id: String,
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = P2PandaNodeDetails),
    (status = 503, description = "Network not started", body = String),
),)]
async fn show_this_panda_node(
    Extension(panda_container): Extension<PandaContainer>,
) -> impl IntoResponse {
    if !panda_container.is_started().await {
        return (StatusCode::SERVICE_UNAVAILABLE, Json("Network not started")).into_response();
    }

    let public_key = panda_container.get_public_key().await.unwrap();
    info!("public key: {:?}", public_key.to_hex());

    let node_details = P2PandaNodeDetails {
        panda_node_id: public_key.to_hex(),
    };

    (StatusCode::OK, Json(node_details)).into_response()
}

#[derive(Serialize, ToSchema)]
pub struct P2PandaLogCount {
    pub node_id: String,
    pub total: i64,
}

#[derive(Serialize, ToSchema)]
struct P2PandaLogCounts {
    pub counts: Vec<P2PandaLogCount>,
}

#[utoipa::path(get, path = "/event_log", responses(
    (status = 200, body = P2PandaLogCounts)
),)]
async fn p2panda_log_counts(
    Extension(panda_container): Extension<PandaContainer>,
) -> impl IntoResponse {
    let counts = panda_container.get_log_counts().await.unwrap_or_else(|e| {
        eprint!("Error finding log count: {}", e);
        vec![]
    });

    let counts = counts
        .into_iter()
        .map(|count| P2PandaLogCount {
            node_id: count.node_id,
            total: count.total,
        })
        .collect();

    (StatusCode::OK, Json(P2PandaLogCounts { counts })).into_response()
}

#[derive(Serialize, ToSchema)]
struct NodeStatusResponse {
    topics: Vec<TopicStatusEntry>,
}

#[derive(Serialize, ToSchema)]
struct TopicStatusEntry {
    topic_hex: String,
    connections: Vec<PeerConnectionEntry>,
}

#[derive(Serialize, ToSchema)]
struct PeerConnectionEntry {
    node_id: String,
    status: PeerConnectionStatus,
}

#[derive(Serialize, ToSchema)]
enum PeerConnectionStatus {
    Unknown,
    Syncing,
    Connected,
    SyncFailed,
}

#[utoipa::path(get, path = "/status", responses(
    (status = 200, body = NodeStatusResponse),
    (status = 503, description = "Network not started", body = String),
),)]
async fn node_status(Extension(panda_container): Extension<PandaContainer>) -> impl IntoResponse {
    match panda_container.get_node_status().await {
        None => (StatusCode::SERVICE_UNAVAILABLE, Json("Network not started")).into_response(),
        Some(snapshot) => {
            let response = NodeStatusResponse {
                topics: snapshot
                    .topics
                    .into_iter()
                    .map(|t| TopicStatusEntry {
                        topic_hex: t.topic_hex,
                        connections: t
                            .connections
                            .into_iter()
                            .map(|c| PeerConnectionEntry {
                                node_id: c.node_id,
                                status: match c.status {
                                    "Syncing" => PeerConnectionStatus::Syncing,
                                    "Connected" => PeerConnectionStatus::Connected,
                                    "SyncFailed" => PeerConnectionStatus::SyncFailed,
                                    _ => PeerConnectionStatus::Unknown,
                                },
                            })
                            .collect(),
                    })
                    .collect(),
            };
            (StatusCode::OK, Json(response)).into_response()
        }
    }
}
