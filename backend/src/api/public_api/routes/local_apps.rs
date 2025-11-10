use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::config_state::LoresNodeConfigState,
    data::{entities::LocalApp, projections_read::nodes::NodesReadRepo},
    local_apps::stack_apps::find_deployed_local_apps,
    DatabaseState,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_local_apps))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<LocalApp>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_local_apps(
    Extension(db): Extension<DatabaseState>,
    Extension(config_state): Extension<LoresNodeConfigState>,
) -> impl IntoResponse {
    let config = config_state.get().await;
    let public_key_hex = match config.public_key_hex {
        Some(key) => key,
        None => {
            eprintln!("No public key hex found in config");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Public key not found").into_response();
        }
    };

    let node_repo = NodesReadRepo::init();
    let node = match node_repo
        .find(&db.projections_pool, public_key_hex.clone())
        .await
    {
        Ok(Some(node)) => node,
        Ok(None) => {
            eprintln!("Node not found for public key: {}", public_key_hex);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Node not found").into_response();
        }
        Err(e) => {
            eprintln!("Failed to find node: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response();
        }
    };

    let apps = find_deployed_local_apps(&node);
    (StatusCode::OK, Json(apps)).into_response()
}
