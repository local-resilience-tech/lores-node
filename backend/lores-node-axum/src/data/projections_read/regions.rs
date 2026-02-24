use sqlx::SqlitePool;

use crate::data::entities::Region;

pub struct RegionsReadRepo {}

impl RegionsReadRepo {
    pub fn init() -> Self {
        RegionsReadRepo {}
    }

    pub async fn find(
        &self,
        pool: &SqlitePool,
        region_id: String,
    ) -> Result<Option<Region>, sqlx::Error> {
        let region = sqlx::query_as!(
            Region,
            "
            SELECT id, creator_node_id, slug, name, organisation_name, url
            FROM regions
            WHERE regions.id = ?
            LIMIT 1
            ",
            region_id
        )
        .fetch_optional(pool)
        .await?;

        return Ok(region);
    }
}
