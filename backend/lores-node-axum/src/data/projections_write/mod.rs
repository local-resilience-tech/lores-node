pub mod app_installations;
pub mod apps;
pub mod current_node_statuses;
pub mod node_statuses;
pub mod nodes;
pub mod region_nodes;
pub mod regions;

use sqlx::{Row, SqlitePool};

pub async fn truncate_all(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Acquire a single connection so that the PRAGMA applies to the same
    // connection as the subsequent DELETEs (PRAGMA foreign_keys is
    // connection-scoped in SQLite).
    let mut conn = pool.acquire().await?;

    sqlx::query("PRAGMA foreign_keys = OFF")
        .execute(&mut *conn)
        .await?;

    let tables: Vec<String> = sqlx::query(
        "SELECT name FROM sqlite_master WHERE type = 'table' AND name != '_sqlx_migrations'",
    )
    .fetch_all(&mut *conn)
    .await?
    .into_iter()
    .map(|row| row.get::<String, _>("name"))
    .collect();

    for table in tables {
        sqlx::query(&format!("DELETE FROM \"{table}\""))
            .execute(&mut *conn)
            .await?;
    }

    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&mut *conn)
        .await?;

    Ok(())
}
