use anyhow::Result;
use sqlx::{migrate::Migrator, sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};
use std::env;
use tower_sessions_sqlx_store::SqliteStore;

lazy_static! {
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:projections.sqlite".to_string());
}

lazy_static! {
    pub static ref NODE_DATA_DATABASE_URL: String = env::var("NODE_DATA_DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:node_data.sqlite".to_string());
}

lazy_static! {
    pub static ref OPERATION_DATABASE_URL: String = env::var("OPERATION_DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:operations.sqlite".to_string());
}

pub async fn prepare_projections_database() -> Result<Pool<Sqlite>> {
    prepare_database(&DATABASE_URL, Some("./migrations_projectiondb")).await
}

pub async fn prepare_node_data_database() -> Result<Pool<Sqlite>> {
    prepare_database(&NODE_DATA_DATABASE_URL, Some("./migrations_nodedatadb")).await
}

pub async fn prepare_session_store(node_data_pool: &SqlitePool) -> Result<SqliteStore> {
    let session_store = SqliteStore::new(node_data_pool.clone());
    session_store.migrate().await?;
    Ok(session_store)
}

pub(crate) async fn prepare_database(
    db_url: &str,
    migrations: Option<&str>,
) -> Result<Pool<Sqlite>> {
    let filename = db_url
        .strip_prefix("sqlite:")
        .ok_or_else(|| anyhow::anyhow!("Database URL must start with 'sqlite:'"))?;

    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    // Fail fast if the SQLite file is read-only. This backend cannot operate
    // correctly without write access to its databases.
    {
        let mut conn = pool.acquire().await?;
        sqlx::query("BEGIN IMMEDIATE").execute(&mut *conn).await.map_err(|e| {
            anyhow::anyhow!(
                "database is not writable ({}): {}. Check file ownership/permissions and mount mode",
                db_url,
                e
            )
        })?;
        sqlx::query("ROLLBACK").execute(&mut *conn).await?;
    }

    if let Some(migrations_path) = migrations {
        println!("Running migrations");
        let migrator = Migrator::new(std::path::Path::new(migrations_path)).await?;
        migrator.run(&pool).await?;
    }

    Ok(pool)
}
