use sqlx::{types::Json, SqlitePool};

use crate::data::entities::{LatLng, Region};

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
        let region = sqlx::query!(
            "
            SELECT
                id,
                creator_node_id,
                slug,
                name,
                organisation_name,
                organisation_url,
                node_steward_conduct_url,
                user_conduct_url,
                user_privacy_url,
                map,
                min_latlng AS \"min_latlng: Json<LatLng>\",
                max_latlng AS \"max_latlng: Json<LatLng>\"
            FROM regions
            WHERE regions.id = ?
            LIMIT 1
            ",
            region_id
        )
        .fetch_optional(pool)
        .await?
        .map(|row| Region {
            id: row.id,
            creator_node_id: row.creator_node_id,
            slug: row.slug,
            name: row.name,
            organisation_name: row.organisation_name,
            organisation_url: row.organisation_url,
            node_steward_conduct_url: row.node_steward_conduct_url,
            user_conduct_url: row.user_conduct_url,
            user_privacy_url: row.user_privacy_url,
            map: row.map,
            min_latlng: row.min_latlng.map(|json| json.0),
            max_latlng: row.max_latlng.map(|json| json.0),
        });

        return Ok(region);
    }

    pub async fn find_all_for_node(
        &self,
        pool: &SqlitePool,
        node_id: &str,
    ) -> Result<Vec<Region>, sqlx::Error> {
        let regions = sqlx::query!(
            "
            SELECT
                r.id,
                r.creator_node_id,
                r.slug,
                r.name,
                r.organisation_name,
                r.organisation_url,
                r.node_steward_conduct_url,
                r.user_conduct_url,
                r.user_privacy_url,
                r.map,
                r.min_latlng AS \"min_latlng: Json<LatLng>\",
                r.max_latlng AS \"max_latlng: Json<LatLng>\"
            FROM regions AS r
            INNER JOIN region_nodes ON r.id = region_nodes.region_id
            WHERE
                region_nodes.node_id = ?
            ",
            node_id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| Region {
            id: row.id,
            creator_node_id: row.creator_node_id,
            slug: row.slug,
            name: row.name,
            organisation_name: row.organisation_name,
            organisation_url: row.organisation_url,
            node_steward_conduct_url: row.node_steward_conduct_url,
            user_conduct_url: row.user_conduct_url,
            user_privacy_url: row.user_privacy_url,
            map: row.map,
            min_latlng: row.min_latlng.map(|json| json.0),
            max_latlng: row.max_latlng.map(|json| json.0),
        })
        .collect();

        return Ok(regions);
    }
}
