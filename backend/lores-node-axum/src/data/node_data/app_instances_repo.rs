use sqlx::SqlitePool;

pub struct AppInstancesRepo {}

impl AppInstancesRepo {
    pub fn init() -> Self {
        AppInstancesRepo {}
    }

    /// Record an app installation binding to a region. Silently ignores rows
    /// that already exist (PRIMARY KEY conflict on app_name + installation_id).
    pub async fn record_app_instance(
        &self,
        pool: &SqlitePool,
        app_name: &str,
        installation_id: &[u8],
        region_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT OR IGNORE INTO app_instances (app_name, installation_id, region_id)
             VALUES (?, ?, ?)",
        )
        .bind(app_name)
        .bind(installation_id)
        .bind(region_id)
        .execute(pool)
        .await?;
        Ok(())
    }
}
