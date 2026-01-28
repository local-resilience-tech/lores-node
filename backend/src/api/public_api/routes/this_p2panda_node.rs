use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::panda_comms::log_access::LogCount;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(show_this_panda_node))
        .routes(routes!(p2panda_log_counts))
}

#[derive(Serialize, ToSchema)]
pub struct IrohNodeAddr {
    pub node_id: String,
    pub relay_url: Option<String>,
    pub direct_addresses: Vec<String>,
}

#[derive(Serialize, ToSchema)]
pub struct PandaNodeAddress {
    pub public_key: String,
    pub direct_addresses: Vec<String>,
    pub relay_url: Option<String>,
}

#[derive(sqlx::FromRow, Serialize, ToSchema)]
pub struct P2PandaNodeDetails {
    pub panda_node_id: String,
    pub iroh_node_addr: IrohNodeAddr,
    pub peers: Vec<PandaNodeAddress>,
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = P2PandaNodeDetails),
    (status = 503, description = "Network not started", body = String),
),)]
async fn show_this_panda_node(//Extension(panda_container): Extension<P2PandaContainer>,
) -> impl IntoResponse {
    return (StatusCode::INTERNAL_SERVER_ERROR, Json("Not implemented")).into_response();

    // @TODO
    // if !panda_container.is_started().await {
    //     return (StatusCode::SERVICE_UNAVAILABLE, Json("Network not started")).into_response();
    // }

    // let public_key: String = panda_container.get_public_key().await.unwrap().to_string();
    // println!("public key: {}", public_key);

    // let node_addr = panda_container.get_node_addr().await;
    // println!("node addr: {:?}", node_addr);

    // let mut peers = panda_container.known_peers().await;

    // if peers.is_err() {
    //     println!("Failed to get known peers {:?}", peers);
    //     peers = Ok(vec![]);
    // } else {
    //     println!("peers: {:?}", peers);
    // }

    // let node_details = P2PandaNodeDetails {
    //     panda_node_id: public_key,
    //     iroh_node_addr: IrohNodeAddr {
    //         node_id: node_addr.node_id.to_string(),
    //         relay_url: node_addr.relay_url.map(|url| url.to_string()),
    //         direct_addresses: node_addr
    //             .direct_addresses
    //             .iter()
    //             .map(|addr| addr.to_string())
    //             .collect(),
    //     },
    //     peers: peers
    //         .unwrap()
    //         .into_iter()
    //         .map(|peer| PandaNodeAddress {
    //             public_key: peer.public_key.to_string(),
    //             direct_addresses: peer
    //                 .direct_addresses
    //                 .iter()
    //                 .map(|addr| addr.to_string())
    //                 .collect(),
    //             relay_url: peer.relay_url.map(|url| url.to_string()),
    //         })
    //         .collect(),
    // };

    // (StatusCode::OK, Json(node_details)).into_response()
}

#[derive(Serialize, ToSchema)]
struct P2PandaLogCounts {
    pub counts: Vec<LogCount>,
}

#[utoipa::path(get, path = "/event_log", responses(
    (status = 200, body = P2PandaLogCounts)
),)]
async fn p2panda_log_counts(// Extension(database_state): Extension<DatabaseState>,
) -> impl IntoResponse {
    // @TODO
    // let operation_pool = database_state.operations_pool.clone();

    // let counts = find_log_count(&operation_pool).await.unwrap_or_else(|e| {
    //     eprint!("Error finding log count: {}", e);
    //     vec![]
    // });

    // (StatusCode::OK, Json(P2PandaLogCounts { counts })).into_response()

    (StatusCode::INTERNAL_SERVER_ERROR, Json("Not implemented")).into_response()
}
