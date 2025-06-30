use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};
use std::env;

lazy_static! {
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").unwrap_or_else(|_| if cfg!(test) {
            "sqlite:main-test.sqlite"
        } else {
            "sqlite:main.sqlite"
        }
        .to_string());
}

pub async fn prepare_database() -> anyhow::Result<Pool<Sqlite>> {
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
    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}
