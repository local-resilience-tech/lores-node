use sqlx::SqlitePool;

pub struct LoresNodeInstallationsWriteRepo {}

impl LoresNodeInstallationsWriteRepo {
    pub fn init() -> Self {
        LoresNodeInstallationsWriteRepo {}
    }

    pub async fn upsert(&self, pool: &SqlitePool, node_id: &str, lores_version: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            INSERT INTO lores_node_installations (node_id, lores_version)
            VALUES(?, ?)
            ON CONFLICT(node_id) DO UPDATE SET lores_version = excluded.lores_version
            ",
            node_id,
            lores_version
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
