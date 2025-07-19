use sqlx::{migrate::Migrator, sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};
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

pub async fn prepare_database(name: &str) -> anyhow::Result<Pool<Sqlite>> {
    // let filename = DATABASE_URL
    //     .strip_prefix("sqlite:")
    //     .ok_or_else(|| anyhow::anyhow!("DATABASE_URL must start with 'sqlite:'"))?;
    let filename = format!("{}.sqlite", name);

    println!("Using database file: {}", filename);

    // create database if it does not exist
    let options = SqliteConnectOptions::new()
        .filename(filename.clone())
        .create_if_missing(true);

    // prepare connection projection_db
    let pool = SqlitePool::connect_with(options).await;

    if pool.is_err() {
        eprintln!(
            "Failed to connect to the database: {}",
            pool.as_ref().err().unwrap()
        );
        return Err(anyhow::anyhow!("Failed to connect to the database"));
    }
    let pool = pool.unwrap();

    // prepare schema in db if it does not yet exist
    let migrations_dir = format!("./migrations/{}", name);
    println!("Running migrations from: {}", migrations_dir);
    let migrator = Migrator::new(std::path::Path::new(&migrations_dir)).await?;
    migrator.run(&pool).await?;

    Ok(pool)
}
