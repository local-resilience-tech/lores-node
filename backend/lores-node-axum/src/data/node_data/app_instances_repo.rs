use sqlx::SqlitePool;

pub struct AppInstancesRepo {}

impl AppInstancesRepo {
    pub fn init() -> Self {
        AppInstancesRepo {}
    }

    /// Record an app installation binding to a region. Silently ignores rows
    /// that already exist.
    pub async fn record_app_instance(
        &self,
        pool: &SqlitePool,
        app_name: &str,
        instance_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT OR IGNORE INTO app_instances (app_name, instance_id)
             VALUES (?, ?)",
        )
        .bind(app_name)
        .bind(instance_id)
        .execute(pool)
        .await?;
        Ok(())
    }
}
