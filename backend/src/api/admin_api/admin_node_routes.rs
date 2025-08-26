use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::config_state::LoresNodeConfigState,
    data::{entities::Node, projections_read::nodes::NodesReadRepo},
    DatabaseState,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(show_this_node))
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
