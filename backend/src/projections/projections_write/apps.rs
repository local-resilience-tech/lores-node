use sqlx::SqlitePool;

use super::super::entities::RegionApp;

pub struct AppsWriteRepo {}

impl AppsWriteRepo {
    pub fn init() -> Self {
        AppsWriteRepo {}
    }

    pub async fn upsert(&self, pool: &SqlitePool, app: RegionApp) -> Result<(), sqlx::Error> {
        let _app = sqlx::query!(
            "
            INSERT INTO apps (name)
            VALUES (?)
            ON CONFLICT(name) DO NOTHING",
            app.name
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
