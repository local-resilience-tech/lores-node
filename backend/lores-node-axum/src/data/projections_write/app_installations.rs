use sqlx::SqlitePool;

use super::super::entities::AppInstallation;

pub struct AppInstallationsWriteRepo {}

impl AppInstallationsWriteRepo {
    pub fn init() -> Self {
        AppInstallationsWriteRepo {}
    }

    pub async fn upsert(
        &self,
        pool: &SqlitePool,
        installation: AppInstallation,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            INSERT INTO app_installations (app_name, node_id, version)
            VALUES (?,?,?)
            ON CONFLICT(app_name, node_id) DO UPDATE SET version = excluded.version",
            installation.app_name,
            installation.node_id,
            installation.version
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
