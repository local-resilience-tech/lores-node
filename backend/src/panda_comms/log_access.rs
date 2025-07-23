use serde::Serialize;
use sqlx::{Row, SqlitePool};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct LogCount {
    pub node_id: String,
    pub total: i64,
}

pub async fn find_log_count(pool: &SqlitePool) -> Result<Vec<LogCount>, sqlx::Error> {
    let result =
        sqlx::query("SELECT public_key, COUNT(*) AS total FROM operations_v1 GROUP BY public_key")
            .fetch_all(pool)
            .await?;

    Ok(result
        .into_iter()
        .map(|row| LogCount {
            node_id: row.get("public_key"),
            total: row.get("total"),
        })
        .collect())
}
