use sqlx::Sqlite;
use thiserror::Error;

pub struct NodeStatusRow {
    pub operation_id: String,
    pub author_node_id: String,
    pub posted_timestamp: u64,
    pub text: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Error, Responder)]
pub enum NodeStatusesError {
    #[error("Internal server error: {0}")]
    #[response(status = 500)]
    InternalServerError(String),
}

pub struct NodeStatusesRepo {}

impl NodeStatusesRepo {
    pub fn init() -> Self {
        NodeStatusesRepo {}
    }

    pub async fn upsert(&self, pool: &sqlx::Pool<Sqlite>, status: NodeStatusRow) -> Result<(), NodeStatusesError> {
        let mut connection = pool.acquire().await.unwrap();

        let timestamp = status.posted_timestamp as i64;

        let _node = sqlx::query!(
            "INSERT INTO node_statuses (operation_id, node_id, text, state, posted_at) VALUES (?, ?, ?, ?, ?) ON CONFLICT(operation_id) DO NOTHING",
            status.operation_id,
            status.author_node_id,
            status.text,
            status.state,
            timestamp
        )
        .execute(&mut *connection)
        .await
        .map_err(|e| NodeStatusesError::InternalServerError(e.to_string()))?;

        Ok(())
    }
}
