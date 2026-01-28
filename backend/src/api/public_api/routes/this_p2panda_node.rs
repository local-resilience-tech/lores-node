use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    panda_comms::{
        log_access::{find_log_count, LogCount},
        panda_node_container::PandaNodeContainer,
    },
    DatabaseState,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(show_this_panda_node))
        .routes(routes!(p2panda_log_counts))
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
    Extension(panda_container): Extension<PandaNodeContainer>,
) -> impl IntoResponse {
    if !panda_container.is_started().await {
        return (StatusCode::SERVICE_UNAVAILABLE, Json("Network not started")).into_response();
    }

    let public_key = panda_container.get_public_key().await.unwrap();
    println!("public key: {:?}", public_key.to_hex());

    let node_details = P2PandaNodeDetails {
        panda_node_id: public_key.to_hex(),
    };

    (StatusCode::OK, Json(node_details)).into_response()
}

#[derive(Serialize, ToSchema)]
struct P2PandaLogCounts {
    pub counts: Vec<LogCount>,
}

#[utoipa::path(get, path = "/event_log", responses(
    (status = 200, body = P2PandaLogCounts)
),)]
async fn p2panda_log_counts(
    Extension(database_state): Extension<DatabaseState>,
) -> impl IntoResponse {
    let operation_pool = database_state.operations_pool.clone();

    let counts = find_log_count(&operation_pool).await.unwrap_or_else(|e| {
        eprint!("Error finding log count: {}", e);
        vec![]
    });

    (StatusCode::OK, Json(P2PandaLogCounts { counts })).into_response()
}
