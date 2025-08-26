use sqlx::SqlitePool;

pub struct NodeStatusRow {
    pub operation_id: String,
    pub author_node_id: String,
    pub posted_timestamp: u64,
    pub text: Option<String>,
    pub state: Option<String>,
}

pub struct NodeStatusesWriteRepo {}

impl NodeStatusesWriteRepo {
    pub fn init() -> Self {
        NodeStatusesWriteRepo {}
    }

    pub async fn upsert(
        &self,
        pool: &SqlitePool,
        status: NodeStatusRow,
    ) -> Result<(), sqlx::Error> {
        let timestamp = status.posted_timestamp as i64;

        let _node = sqlx::query!(
            "
            INSERT INTO node_statuses (operation_id, node_id, text, state, posted_at)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(operation_id) DO NOTHING",
            status.operation_id,
            status.author_node_id,
            status.text,
            status.state,
            timestamp
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
