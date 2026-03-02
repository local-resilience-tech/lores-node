use sqlx::SqlitePool;

pub struct AppsWriteRepo {}

impl AppsWriteRepo {
    pub fn init() -> Self {
        AppsWriteRepo {}
    }

    pub async fn upsert(&self, pool: &SqlitePool, app_name: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            INSERT INTO apps (name)
            VALUES (?)
            ON CONFLICT(name) DO NOTHING",
            app_name
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
