use rocket::{fairing, Build, Rocket};
use rocket_db_pools::Database;
use sqlx::Sqlite;

#[derive(Database)]
#[database("main_db")]
pub struct MainDb(sqlx::SqlitePool);

impl MainDb {
    pub fn sqlite_pool(&self) -> &sqlx::Pool<Sqlite> {
        &self.0
    }
}

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    if let Some(db) = MainDb::fetch(&rocket) {
        println!("Running DB migrations");

        // run migrations using `db`. get the inner type with &db.0.
        sqlx::migrate!()
            .run(&db.0)
            .await
            .expect("Error running DB migrations");

        Ok(rocket)
    } else {
        Err(rocket)
    }
}
