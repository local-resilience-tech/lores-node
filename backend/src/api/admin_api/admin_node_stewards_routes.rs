use axum::{extract, http::StatusCode, response::IntoResponse, Extension, Json};
use npwg::{generate_password_with_config, PasswordGeneratorConfig, PasswordGeneratorError};
use serde::{Deserialize, Serialize};
use short_uuid::ShortUuid;
use std::collections::HashSet;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    data::node_data::node_stewards::{NodeStewardRow, NodeStewardsRepo},
    DatabaseState,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_node_stewards))
        .routes(routes!(create_node_steward))
}

#[derive(Serialize, ToSchema)]
pub struct NodeSteward {
    pub id: String,
    pub name: String,
    pub active: bool,
}

#[utoipa::path(get, path = "/",
    responses(
        (status = OK, body = Vec<NodeSteward>),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    ),
)]
async fn list_node_stewards(Extension(db): Extension<DatabaseState>) -> impl IntoResponse {
    let repo = NodeStewardsRepo::init();

    match repo.all(&db.node_data_pool).await {
        Ok(nodes) => {
            let result_nodes: Vec<NodeSteward> = nodes
                .into_iter()
                .map(|node| NodeSteward {
                    id: node.id,
                    name: node.name,
                    active: node.active,
                })
                .collect();
            (StatusCode::OK, Json(result_nodes)).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response(),
    }
}

#[derive(Deserialize, ToSchema)]
pub struct NodeStewardCreationData {
    pub name: String,
}

#[derive(Serialize, ToSchema)]
pub struct NodeStewardCreationResult {
    pub node_steward: NodeSteward,
    pub password_reset_token: String,
}

#[utoipa::path(post, path = "/",
    request_body(content = NodeStewardCreationData, content_type = "application/json"),
    responses(
        (status = CREATED, body = Vec<NodeSteward>),
        (status = INTERNAL_SERVER_ERROR, body = String),
    ),
)]
async fn create_node_steward(
    Extension(db): Extension<DatabaseState>,
    extract::Json(input): extract::Json<NodeStewardCreationData>,
) -> impl IntoResponse {
    let token = match new_password_reset_token().await {
        Ok(token) => token,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error generating password reset token".to_string(),
            )
                .into_response()
        }
    };

    let new_row = NodeStewardRow {
        id: new_node_steward_id(),
        name: input.name,
        hashed_password: None,
        password_reset_token: Some(token),
        active: true,
    };

    let repo = NodeStewardsRepo::init();
    let result = repo.create(&db.node_data_pool, &new_row).await;

    match result {
        Ok(_) => {
            let creation_result = NodeStewardCreationResult {
                node_steward: NodeSteward {
                    id: new_row.id,
                    name: new_row.name,
                    active: new_row.active,
                },
                password_reset_token: new_row.password_reset_token.unwrap_or_default(),
            };
            (StatusCode::CREATED, Json(creation_result)).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "server error".to_string(),
        )
            .into_response(),
    }
}

fn new_node_steward_id() -> String {
    ShortUuid::generate().to_string()
}

async fn new_password_reset_token() -> Result<String, PasswordGeneratorError> {
    let mut pw_config = PasswordGeneratorConfig::new();
    pw_config.length = 8;
    pw_config.add_allowed_chars("digit");
    pw_config.add_allowed_chars("lowerletter");
    pw_config.add_allowed_chars("upperletter");
    pw_config.excluded_chars = HashSet::from(['0', 'O']);

    generate_password_with_config(&pw_config).await
}
