use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::panda_comms::container::P2PandaContainer;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(show_this_panda_node))
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
pub struct NodeDetails {
    pub panda_node_id: String,
    pub iroh_node_addr: IrohNodeAddr,
    pub peers: Vec<PandaNodeAddress>,
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = NodeDetails),
    (status = 503, description = "Network not started", body = String),
),)]
async fn show_this_panda_node(
    Extension(panda_container): Extension<P2PandaContainer>,
) -> impl IntoResponse {
    if !panda_container.is_started().await {
        return (StatusCode::SERVICE_UNAVAILABLE, Json("Network not started")).into_response();
    }

    let public_key: String = panda_container.get_public_key().await.unwrap().to_string();
    println!("public key: {}", public_key);

    let node_addr = panda_container.get_node_addr().await;
    println!("node addr: {:?}", node_addr);

    let mut peers = panda_container.known_peers().await;

    if peers.is_err() {
        println!("Failed to get known peers {:?}", peers);
        peers = Ok(vec![]);
    } else {
        println!("peers: {:?}", peers);
    }

    let node_details = NodeDetails {
        panda_node_id: public_key,
        iroh_node_addr: IrohNodeAddr {
            node_id: node_addr.node_id.to_string(),
            relay_url: node_addr.relay_url.map(|url| url.to_string()),
            direct_addresses: node_addr
                .direct_addresses
                .iter()
                .map(|addr| addr.to_string())
                .collect(),
        },
        peers: peers
            .unwrap()
            .into_iter()
            .map(|peer| PandaNodeAddress {
                public_key: peer.public_key.to_string(),
                direct_addresses: peer
                    .direct_addresses
                    .iter()
                    .map(|addr| addr.to_string())
                    .collect(),
                relay_url: peer.relay_url.map(|url| url.to_string()),
            })
            .collect(),
    };

    (StatusCode::OK, Json(node_details)).into_response()
}

#[derive(Deserialize, ToSchema)]
pub struct BootstrapNodePeer {
    pub node_id: String,
}

#[derive(Deserialize, ToSchema)]
pub struct BootstrapNodeData {
    pub network_name: String,
    pub bootstrap_peer: Option<BootstrapNodePeer>,
}
