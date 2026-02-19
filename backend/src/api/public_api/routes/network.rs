use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    data::entities::{Network, NetworkNode},
    panda_comms::panda_node_container::PandaNodeContainer,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(show_network))
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, body = Network),
        (status = 500, body = String),
    )
)]
async fn show_network(
    Extension(panda_container): Extension<PandaNodeContainer>,
) -> impl IntoResponse {
    if !panda_container.is_started().await {
        return (StatusCode::SERVICE_UNAVAILABLE, Json("Network not started")).into_response();
    }

    let params = panda_container.get_params().await;

    let network_name = match params.network_name {
        Some(name) => name,
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Network name not found"),
            )
                .into_response()
        }
    };

    let public_key = panda_container.get_public_key().await.unwrap();

    let node = NetworkNode {
        id: public_key.to_hex(),
    };
    let result = Network {
        name: network_name,
        node,
    };

    (StatusCode::OK, Json(result)).into_response()
}
