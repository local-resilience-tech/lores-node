use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use p2panda_core::PublicKey;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    admin_api::routes::this_p2panda_node::BootstrapNodeData,
    config::LoresNodeConfig,
    panda_comms::{
        config::{SimplifiedNodeAddress, ThisP2PandaNodeRepo},
        container::{build_public_key_from_hex, P2PandaContainer},
    },
    projections::entities::Region,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(show_region))
        .routes(routes!(bootstrap))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Option<Region>, description = "Returns the current region's network ID if available"),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn show_region(Extension(config): Extension<LoresNodeConfig>) -> impl IntoResponse {
    match config.network_name {
        Some(network_id) => {
            println!("got network id {}", network_id);
            (StatusCode::OK, Json(Some(Region { network_id })))
        }
        None => {
            println!("no network id");
            (StatusCode::OK, Json(None))
        }
    }
    .into_response()
}

#[utoipa::path(
    post,
    path = "/bootstrap",
    request_body(content = BootstrapNodeData, content_type = "application/json"),
    responses(
        (status = 200, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = String),
    )
)]
async fn bootstrap(
    Extension(mut config): Extension<LoresNodeConfig>,
    Extension(panda_container): Extension<P2PandaContainer>,
    axum::extract::Json(data): axum::extract::Json<BootstrapNodeData>,
) -> impl IntoResponse {
    let repo = ThisP2PandaNodeRepo::init();

    let peer_address: Option<SimplifiedNodeAddress> =
        data.node_id.as_ref().map(|node_id| SimplifiedNodeAddress {
            node_id: node_id.clone(),
        });

    repo.set_network_config(&mut config, data.network_name.clone(), peer_address.clone());

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
