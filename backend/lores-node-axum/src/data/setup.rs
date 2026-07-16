use anyhow::Result;
use sqlx::{Pool, Sqlite, SqlitePool, migrate::Migrator, sqlite::SqliteConnectOptions};
use std::env;
use tower_sessions_sqlx_store::SqliteStore;
use tracing::info;

lazy_static! {
    static ref DATA_DIR: String = env::var("DATA_DIR").unwrap_or_else(|_| ".".to_string());
}

fn db_url(name: &str, version: Option<u32>) -> String {
    let file_basename = match version {
        Some(v) => format!("{}-{}", name, v),
        None => name.to_string(),
    };
    format!("sqlite:{}/{}.sqlite", *DATA_DIR, file_basename)
}

const OPERATIONS_DATABASE_VERSION: u32 = 1;

fn database_url() -> String {
    db_url("projections", Some(OPERATIONS_DATABASE_VERSION))
}
fn node_data_database_url() -> String {
    db_url("node_data", None)
}
pub fn operation_database_url() -> String {
    db_url("operations", Some(OPERATIONS_DATABASE_VERSION))
}

pub async fn prepare_projections_database() -> Result<Pool<Sqlite>> {
    prepare_database(&database_url(), Some("./migrations_projectiondb")).await
}

pub async fn prepare_node_data_database() -> Result<Pool<Sqlite>> {
    prepare_database(&node_data_database_url(), Some("./migrations_nodedatadb")).await
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
        info!("Running migrations");
        let migrator = Migrator::new(std::path::Path::new(migrations_path)).await?;
        migrator.run(&pool).await?;
    }

    Ok(pool)
}
