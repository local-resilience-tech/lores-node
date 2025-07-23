use sqlx::SqlitePool;

use crate::projections::entities::RegionApp;

pub struct AppsReadRepo {}

impl AppsReadRepo {
    pub fn init() -> Self {
        AppsReadRepo {}
    }

    pub async fn all(&self, pool: &SqlitePool) -> Result<Vec<RegionApp>, sqlx::Error> {
        sqlx::query_as!(RegionApp, "SELECT name FROM apps")
            .fetch_all(pool)
            .await
    }
}
