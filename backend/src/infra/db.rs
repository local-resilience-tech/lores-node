use p2panda_store::sqlite::store::run_pending_migrations;
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};
use std::env;

lazy_static! {
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:projections.sqlite".to_string());
}

lazy_static! {
    pub static ref OPERATION_DATABASE_URL: String = env::var("OPERATION_DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:operations.sqlite".to_string());
}

pub async fn prepare_projections_database() -> anyhow::Result<Pool<Sqlite>> {
    let filename = DATABASE_URL
        .strip_prefix("sqlite:")
        .ok_or_else(|| anyhow::anyhow!("DATABASE_URL must start with 'sqlite:'"))?;

    println!("Using database file: {}", filename);

    // create database if it does not exist
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    // prepare connection pool
    let pool = SqlitePool::connect_with(options).await?;

    // prepare schema in db if it does not yet exist
    println!("Running migrations");
    sqlx::migrate!("./migrations_projectiondb")
        .run(&pool)
        .await?;

    Ok(pool)
}

pub async fn prepare_operations_database() -> anyhow::Result<Pool<Sqlite>> {
    let filename = OPERATION_DATABASE_URL
        .strip_prefix("sqlite:")
        .ok_or_else(|| anyhow::anyhow!("OPERATION_DATABASE_URL must start with 'sqlite:'"))?;

    println!("Using operation database file: {}", filename);

    // create database if it does not exist
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    // prepare connection pool
    let pool = SqlitePool::connect_with(options).await?;

    run_pending_migrations(&pool).await?;

    Ok(pool)
}
