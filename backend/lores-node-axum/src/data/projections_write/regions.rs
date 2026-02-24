use sqlx::SqlitePool;

use crate::data::entities::Region;

pub struct RegionsWriteRepo {}

impl RegionsWriteRepo {
    pub fn init() -> Self {
        RegionsWriteRepo {}
    }

    pub async fn insert(&self, pool: &SqlitePool, region: &Region) -> Result<(), sqlx::Error> {
        let _region = sqlx::query!(
            "INSERT INTO regions (
                id, creator_node_id, slug, name, organisation_name, url
            )
            VALUES (?, ?, ?, ?, ?, ?)",
            region.id,
            region.creator_node_id,
            region.slug,
            region.name,
            region.organisation_name,
            region.url
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
