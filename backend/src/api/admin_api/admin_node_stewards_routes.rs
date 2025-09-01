use axum::{
    extract::{self, Path},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    data::node_data::node_stewards::{NodeStewardIdentifier, NodeStewardRow, NodeStewardsRepo},
    DatabaseState,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(list_node_stewards))
        .routes(routes!(create_node_steward))
        .routes(routes!(reset_node_steward_token))
        .routes(routes!(disable_node_steward))
        .routes(routes!(enable_node_steward))
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
    let mut new_row = NodeStewardRow::new(input.name);
    new_row.set_password_reset_token();

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

#[utoipa::path(
    post,
    path = "/reset_token/{steward_id}",
    params(
        ("steward_id" = String, Path),
    ),
    responses(
        (status = OK, body = NodeStewardCreationResult),
        (status = NOT_FOUND, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    ),
)]
async fn reset_node_steward_token(
    Extension(db): Extension<DatabaseState>,
    Path(steward_id): Path<String>,
) -> impl IntoResponse {
    let repo = NodeStewardsRepo::init();
    let identifier = NodeStewardIdentifier { id: steward_id };

    let mut row = match repo.find(&db.node_data_pool, &identifier).await {
        Ok(Some(row)) => row,
        Ok(None) => return (StatusCode::NOT_FOUND, ()).into_response(),
        Err(e) => {
            eprintln!("Error finding node steward: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response();
        }
    };
    row.set_password_reset_token();

    let result = repo
        .update_password_reset_token(&db.node_data_pool, &row)
        .await;

    match result {
        Ok(_) => {
            let creation_result = NodeStewardCreationResult {
                node_steward: NodeSteward::from_row(&row),
                password_reset_token: row.password_reset_token.unwrap_or_default(),
            };
            (StatusCode::OK, Json(creation_result)).into_response()
        }
        Err(e) => {
            eprintln!("Error saving reset token: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response()
        }
    }
}

#[utoipa::path(
    post,
    path = "/disable/{steward_id}",
    params(
        ("steward_id" = String, Path),
    ),
    responses(
        (status = OK, body = NodeSteward),
        (status = NOT_FOUND, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    ),
)]
async fn disable_node_steward(
    Extension(db): Extension<DatabaseState>,
    Path(steward_id): Path<String>,
) -> impl IntoResponse {
    toggle_node_steward_status(db, steward_id, false).await
}

#[utoipa::path(
    post,
    path = "/enable/{steward_id}",
    params(
        ("steward_id" = String, Path),
    ),
    responses(
        (status = OK, body = NodeSteward),
        (status = NOT_FOUND, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    ),
)]
async fn enable_node_steward(
    Extension(db): Extension<DatabaseState>,
    Path(steward_id): Path<String>,
) -> impl IntoResponse {
    toggle_node_steward_status(db, steward_id, true).await
}

async fn toggle_node_steward_status(
    db: DatabaseState,
    steward_id: String,
    enabled: bool,
) -> impl IntoResponse {
    let repo = NodeStewardsRepo::init();
    let identifier = NodeStewardIdentifier { id: steward_id };

    let result = repo
        .update_enabled(&db.node_data_pool, &identifier, enabled)
        .await;

    if let Err(e) = result {
        eprintln!("Error updating node steward status: {:?}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response();
    }

    match repo.find(&db.node_data_pool, &identifier).await {
        Ok(Some(row)) => (StatusCode::OK, Json(NodeSteward::from_row(&row))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, ()).into_response(),
        Err(e) => {
            eprintln!("Error finding node steward: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response()
        }
    }
}
