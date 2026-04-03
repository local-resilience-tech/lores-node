use p2panda_store::{SqliteError, SqliteStore};
use sqlx::Row;

pub struct LogCount {
    pub node_id: String,
    pub total: i64,
}

async fn query_log_count(pool: &sqlx::SqlitePool) -> Result<Vec<LogCount>, SqliteError> {
    let result =
        sqlx::query("SELECT public_key, COUNT(*) AS total FROM operations_v1 GROUP BY public_key")
            .fetch_all(pool)
            .await?;

    Ok(result
        .into_iter()
        .map(|row: sqlx::sqlite::SqliteRow| LogCount {
            node_id: row.get("public_key"),
            total: row.get("total"),
        })
        .collect())
}

pub async fn find_log_count(store: &SqliteStore) -> Result<Vec<LogCount>, SqliteError> {
    store.execute(query_log_count).await
}
