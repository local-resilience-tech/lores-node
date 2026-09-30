use sqlx::SqlitePool;

use crate::data::entities::LoresNode;

pub struct LoresNodeInstallationsReadRepo {}

impl LoresNodeInstallationsReadRepo {
    pub fn init() -> Self {
        LoresNodeInstallationsReadRepo {}
    }

    pub async fn find_by_node_id(&self, pool: &SqlitePool, node_id: &str) -> Result<Option<LoresNode>, sqlx::Error> {
        let node = sqlx::query_as!(
            LoresNode,
            "
            SELECT node_id, lores_version
            FROM lores_node_installations
            LIMIT 1
            ",
            node_id,
        )
        .fetch_optional(pool)
        .await?;

        Ok(node)
    }
}
