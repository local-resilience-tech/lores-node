use axum::{extract, http::StatusCode, response::IntoResponse, Extension, Json};
use chrono::NaiveDateTime;
use pwgen2::pwgen::{generate_password, PasswordConfig};
use serde::{Deserialize, Serialize};
use short_uuid::ShortUuid;
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
pub enum NodeStewardStatus {
    Enabled,
    Disabled,
    Invited,
    TokenExpired,
}

#[derive(Serialize, ToSchema)]
pub struct NodeSteward {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub status: NodeStewardStatus,
}

impl NodeSteward {
    pub fn from_row(row: &NodeStewardRow) -> Self {
        let mut status: NodeStewardStatus = match row.enabled {
            true => NodeStewardStatus::Enabled,
            false => NodeStewardStatus::Disabled,
        };
        if row.hashed_password.is_none() {
            match row.token_expired() {
                true => status = NodeStewardStatus::TokenExpired,
                false => status = NodeStewardStatus::Invited,
            }
        }

        NodeSteward {
            id: row.id.clone(),
            name: row.name.clone(),
            created_at: row.created_at.unwrap_or_default().to_string(),
            status,
        }
    }
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
        Ok(stewards) => {
            let results: Vec<NodeSteward> = stewards
                .into_iter()
                .map(|steward| NodeSteward::from_row(&steward))
                .collect();
            (StatusCode::OK, Json(results)).into_response()
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
        (status = CREATED, body = NodeStewardCreationResult),
        (status = INTERNAL_SERVER_ERROR, body = String),
    ),
)]
async fn create_node_steward(
    Extension(db): Extension<DatabaseState>,
    extract::Json(input): extract::Json<NodeStewardCreationData>,
) -> impl IntoResponse {
    let new_row = NodeStewardRow {
        id: new_node_steward_id(),
        name: input.name,
        hashed_password: None,
        password_reset_token: Some(new_password_reset_token()),
        password_reset_token_expires_at: Some(new_reset_token_expiry()),
        enabled: true,
        created_at: None,
    };

    let repo = NodeStewardsRepo::init();
    let result = repo.create(&db.node_data_pool, &new_row).await;

    match result {
        Ok(_) => {
            let creation_result = NodeStewardCreationResult {
                node_steward: NodeSteward::from_row(&new_row),
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

fn new_password_reset_token() -> String {
    let pw_config = PasswordConfig::alphanumeric(8).unwrap();
    generate_password(&pw_config)
}

fn new_reset_token_expiry() -> NaiveDateTime {
    chrono::Utc::now().naive_utc() + chrono::Duration::hours(24)
}
