use sqlx::SqlitePool;

pub struct CurrentNodeStatusRow {
    pub region_node_id: i64,
    pub posted_timestamp: u64,
    pub text: Option<String>,
    pub state: Option<String>,
}

pub struct CurrentNodeStatusesWriteRepo {}

impl CurrentNodeStatusesWriteRepo {
    pub fn init() -> Self {
        CurrentNodeStatusesWriteRepo {}
    }

    pub async fn upsert(
        &self,
        pool: &SqlitePool,
        status: CurrentNodeStatusRow,
    ) -> Result<(), sqlx::Error> {
        let timestamp = status.posted_timestamp as i64;

        let _node = sqlx::query!(
            "
            INSERT INTO current_node_statuses (region_node_id, text, state, posted_at)
            VALUES (?, ?, ?, ?)
            ON CONFLICT(region_node_id) DO UPDATE SET text = excluded.text, state = excluded.state, posted_at = excluded.posted_at",
            status.region_node_id,
            status.text,
            status.state,
            timestamp,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
