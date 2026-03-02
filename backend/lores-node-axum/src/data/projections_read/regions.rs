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
        region_id: &str,
    ) -> Result<Option<Region>, sqlx::Error> {
        let region = sqlx::query_as!(
            Region,
            "
            SELECT
                id, creator_node_id, slug, name, organisation_name, organisation_url, node_steward_conduct_url, user_conduct_url, user_privacy_url
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

    pub async fn find_all_for_node(
        &self,
        pool: &SqlitePool,
        node_id: &str,
    ) -> Result<Vec<Region>, sqlx::Error> {
        let regions = sqlx::query_as!(
            Region,
            "
            SELECT
                r.id, r.creator_node_id, r.slug, r.name,
                r.organisation_name, r.organisation_url,
                r.node_steward_conduct_url, r.user_conduct_url,
                r.user_privacy_url
            FROM regions AS r
            INNER JOIN region_nodes ON r.id = region_nodes.region_id
            WHERE
                region_nodes.node_id = ?
            ",
            node_id
        )
        .fetch_all(pool)
        .await?;

        return Ok(regions);
    }
}
