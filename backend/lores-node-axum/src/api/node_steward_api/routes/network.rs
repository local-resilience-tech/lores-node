use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use tracing::warn;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    config::config_state::LoresNodeConfigState,
    data::projections_write::truncate_all,
    panda_comms::{build_public_key_from_hex, PandaContainer},
    DatabaseState,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(add_bootstrap_node))
        .routes(routes!(replay_projections))
        .routes(routes!(get_operation_counts))
}

#[derive(Deserialize, ToSchema, Debug)]
struct BootstrapNodeRequest {
    node_id: String,
}

impl BootstrapNodeRequest {
    fn validate(&self) -> Result<(), String> {
        build_public_key_from_hex(&self.node_id).map_err(|_| {
            "Invalid node_id format. Must be a hex string representing a public key.".to_string()
        })?;

        Ok(())
    }
}

#[utoipa::path(
    post,
    path = "/bootstrap",
    request_body = BootstrapNodeRequest,
    responses(
        (status = 200, body = ()),
        (status = 500, body = String),
    )
)]
async fn add_bootstrap_node(
    Extension(panda_container): Extension<PandaContainer>,
    Extension(config_state): Extension<LoresNodeConfigState>,
    axum::Json(payload): axum::Json<BootstrapNodeRequest>,
) -> impl IntoResponse {
    if let Err(err) = payload.validate() {
        return (StatusCode::BAD_REQUEST, err).into_response();
    }

    // Add the bootstrap node to the current PandaContainer
    if let Err(e) = panda_container.add_bootstrap_node(&payload.node_id).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to add bootstrap node: {}", e),
        )
            .into_response();
    }

    // Add the new bootstrap node ID to the config
    let update_result = config_state
        .update(|config| {
            let mut result = config.clone();
            let mut bootstrap_node_ids = result.bootstrap_node_ids.unwrap_or_default();
            if !bootstrap_node_ids.contains(&payload.node_id) {
                bootstrap_node_ids.push(payload.node_id.clone());
            }
            result.bootstrap_node_ids = Some(bootstrap_node_ids);
            result
        })
        .await;
    if let Err(e) = update_result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update config: {}", e),
        )
            .into_response();
    }

    (StatusCode::OK, ()).into_response()
}

#[utoipa::path(
    post,
    path = "/replay",
    responses(
        (status = 200, body = String),
        (status = 500, body = String),
    )
)]
async fn replay_projections(
    Extension(db): Extension<DatabaseState>,
    Extension(panda_container): Extension<PandaContainer>,
) -> impl IntoResponse {
    if let Err(e) = truncate_all(&db.projections_pool).await {
        warn!("Failed to truncate projection tables: {e}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to truncate projection tables: {e}"),
        )
            .into_response();
    }

    match panda_container.replay_all_regions().await {
        Ok(count) => (
            StatusCode::OK,
            format!("Replaying {count} region(s) from the operations store"),
        )
            .into_response(),
        Err(e) => {
            warn!("Failed to start replay: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to start replay: {e}"),
            )
                .into_response()
        }
    }
}

#[derive(Serialize, ToSchema)]
struct OperationCountEntry {
    topic: String,
    author_node_id: String,
    count: i64,
}

#[utoipa::path(
    get,
    path = "/operations/counts",
    responses(
        (status = 200, body = Vec<OperationCountEntry>),
        (status = 500, body = String),
    )
)]
async fn get_operation_counts(
    Extension(panda_container): Extension<PandaContainer>,
) -> impl IntoResponse {
    match panda_container.get_operation_counts_by_topic().await {
        Ok(counts) => {
            let entries: Vec<OperationCountEntry> = counts
                .into_iter()
                .map(|c| OperationCountEntry {
                    topic: c.topic_hex,
                    author_node_id: c.author_node_id,
                    count: c.count,
                })
                .collect();
            Json(entries).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
