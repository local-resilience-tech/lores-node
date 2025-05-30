use super::entities::Node;
use crate::{infra::db::MainDb, repos::helpers::NODE_CONFIG_ID};
use rocket_db_pools::Connection;
use thiserror::Error;

pub struct ThisNodeRepo {}

#[derive(Debug, Error, Responder)]
pub enum ThisNodeRepoError {
    #[error("Internal server error: {0}")]
    #[response(status = 500)]
    InternalServerError(String),

    #[error("Node not found")]
    #[response(status = 404)]
    NotFound(String),
}

impl ThisNodeRepo {
    pub fn init() -> Self {
        ThisNodeRepo {}
    }

    pub async fn find(&self, db: &mut Connection<MainDb>) -> Result<Node, ThisNodeRepoError> {
        let node = sqlx::query_as!(
            Node,
            "
            SELECT nodes.id as id, nodes.name as name
            FROM nodes
            INNER JOIN node_configs ON node_configs.public_key_hex = nodes.id
            WHERE node_configs.id = ? LIMIT 1
            ",
            NODE_CONFIG_ID
        )
        .fetch_one(&mut ***db)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ThisNodeRepoError::NotFound("Node not found".to_string()),
            _ => ThisNodeRepoError::InternalServerError("Database error".to_string()),
        })?;

        return Ok(node);
    }
}
