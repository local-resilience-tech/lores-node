use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use p2panda_core::PublicKey;
use sqlx::SqlitePool;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    panda_comms::container::{build_public_key_from_hex, P2PandaContainer},
    repos::{
        entities::{NodeDetails, Region},
        nodes::NodesRepo,
        this_p2panda_node::{SimplifiedNodeAddress, ThisP2PandaNodeRepo},
    },
    routes::this_p2panda_node::BootstrapNodeData,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(show_region))
        .routes(routes!(nodes))
        .routes(routes!(bootstrap))
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

#[utoipa::path(
    post,
    path = "/bootstrap",
    responses(
        (status = 200, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = String),
    )
)]
async fn bootstrap(
    Extension(pool): Extension<SqlitePool>,
    Extension(panda_container): Extension<P2PandaContainer>,
    axum::extract::Json(data): axum::extract::Json<BootstrapNodeData>,
) -> impl IntoResponse {
    let repo = ThisP2PandaNodeRepo::init();

    let bootstrap_peer = &data.bootstrap_peer;

    let peer_address: Option<SimplifiedNodeAddress> =
        bootstrap_peer.as_ref().map(|peer| SimplifiedNodeAddress {
            node_id: peer.node_id.clone(),
        });

    let result = repo
        .set_network_config(&pool, data.network_name.clone(), peer_address.clone())
        .await;
    if let Err(e) = result {
        eprintln!("Failed to set network config: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to set network config".to_string(),
        )
            .into_response();
    }

    panda_container
        .set_network_name(data.network_name.clone())
        .await;

    let bootstrap_node_id: Option<PublicKey> = match peer_address.clone() {
        Some(bootstrap) => build_public_key_from_hex(bootstrap.node_id.clone()),
        None => None,
    };
    panda_container
        .set_bootstrap_node_id(bootstrap_node_id)
        .await;

    // start the container
    if let Err(e) = panda_container.start().await {
        eprintln!("Failed to start P2PandaContainer: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Failed to start P2PandaContainer".to_string()),
        )
            .into_response();
    }

    (StatusCode::OK, ()).into_response()
}
