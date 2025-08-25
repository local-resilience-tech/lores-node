use anyhow::Result;
use p2panda_store::sqlite::store::run_pending_migrations;
use sqlx::{migrate::Migrator, sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};
use std::env;

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

pub async fn prepare_operations_database() -> Result<Pool<Sqlite>> {
    let pool = prepare_database(&OPERATION_DATABASE_URL, None).await?;

    run_pending_migrations(&pool)
        .await
        .map_err(anyhow::Error::from)?;

    Ok(pool)
}

pub async fn prepare_node_data_database() -> Result<Pool<Sqlite>> {
    prepare_database(&NODE_DATA_DATABASE_URL, Some("./migrations_nodedatadb")).await
}

async fn prepare_database(db_url: &str, migrations: Option<&str>) -> Result<Pool<Sqlite>> {
    let filename = db_url
        .strip_prefix("sqlite:")
        .ok_or_else(|| anyhow::anyhow!("Database URL must start with 'sqlite:'"))?;

    println!("Using database file: {}", filename);

    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    if let Some(migrations_path) = migrations {
        println!("Running migrations");
        let migrator = Migrator::new(std::path::Path::new(migrations_path)).await?;
        migrator.run(&pool).await?;
    }

    Ok(pool)
}
