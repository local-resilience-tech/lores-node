use rocket_db_pools::Connection;
use sqlx::Sqlite;
use thiserror::Error;

use crate::infra::db::MainDb;

use super::entities::{Node, NodeDetails};

pub struct NodesRepo {}

#[derive(Debug, Error, Responder)]
pub enum NodesError {
    #[error("Internal server error: {0}")]
    #[response(status = 500)]
    InternalServerError(String),
    // #[error("Cannot create node")]
    // #[response(status = 409)]
    // CannotCreate(String),

    // #[error("Node not found")]
    // #[response(status = 404)]
    // NotFound(String),
}

impl NodesRepo {
    pub fn init() -> Self {
        NodesRepo {}
    }

    pub async fn upsert(&self, pool: &sqlx::Pool<Sqlite>, node: Node) -> Result<(), NodesError> {
        let mut connection = pool.acquire().await.unwrap();

        let _node = sqlx::query!(
            "INSERT INTO nodes (id, name) VALUES (?, ?) ON CONFLICT(id) DO UPDATE SET name = ?",
            node.id,
            node.name,
            node.name
        )
        .execute(&mut *connection)
        .await
        .map_err(|_| NodesError::InternalServerError("Database error".to_string()))?;

        Ok(())
    }

    pub async fn update(&self, pool: &sqlx::Pool<Sqlite>, node: NodeDetails) -> Result<(), NodesError> {
        let mut connection = pool.acquire().await.unwrap();

        let _node = sqlx::query!(
            "UPDATE nodes SET name = ?, public_ipv4 = ? WHERE id = ?",
            node.name,
            node.public_ipv4,
            node.id
        )
        .execute(&mut *connection)
        .await
        .map_err(|_| NodesError::InternalServerError("Database error".to_string()))?;

        Ok(())
    }

    pub async fn all(&self, connection: &mut Connection<MainDb>) -> Result<Vec<Node>, NodesError> {
        let nodes = sqlx::query_as!(Node, "SELECT id, name FROM nodes")
            .fetch_all(&mut ***connection)
            .await
            .map_err(|_| NodesError::InternalServerError("Database error".to_string()))?;

        Ok(nodes)
    }
}
