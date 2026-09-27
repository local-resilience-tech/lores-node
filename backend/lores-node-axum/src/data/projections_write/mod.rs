pub mod app_installations;
pub mod apps;
pub mod current_node_statuses;
pub mod node_statuses;
pub mod nodes;
pub mod region_nodes;
pub mod regions;

use sqlx::{AssertSqlSafe, SqlitePool};

pub async fn truncate_all(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Acquire a single connection so that the PRAGMA applies to the same
    // connection as the subsequent DELETEs (PRAGMA foreign_keys is
    // connection-scoped in SQLite).
    let mut conn = pool.acquire().await?;

    sqlx::query("PRAGMA foreign_keys = OFF").execute(&mut *conn).await?;

    let tables: Vec<String> = sqlx::query_scalar::<_, String>(
        "SELECT name FROM sqlite_master WHERE type = 'table' AND name != '_sqlx_migrations'",
    )
    .fetch_all(&mut *conn)
    .await?;

    if !tables.is_empty() {
        let deletes = tables
            .iter()
            .map(|table| format!("DELETE FROM \"{}\";", table.replace('"', "\"\"")))
            .collect::<Vec<_>>()
            .join("\n");

        sqlx::raw_sql(AssertSqlSafe(deletes)).execute(&mut *conn).await?;
    }

    sqlx::query("PRAGMA foreign_keys = ON").execute(&mut *conn).await?;

    Ok(())
}
