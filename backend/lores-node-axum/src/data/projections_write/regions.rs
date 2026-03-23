use sqlx::{types::Json, SqlitePool};

use crate::{
    data::entities::{LatLng, Region},
    panda_comms::RegionId,
};

pub struct RegionsWriteRepo {}

impl RegionsWriteRepo {
    pub fn init() -> Self {
        RegionsWriteRepo {}
    }

    pub async fn upsert(&self, pool: &SqlitePool, region: &Region) -> Result<(), sqlx::Error> {
        let _region = sqlx::query!(
            "INSERT INTO regions (
                id, creator_node_id, slug, name, organisation_name,
                organisation_url, node_steward_conduct_url, user_conduct_url, user_privacy_url
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                creator_node_id = excluded.creator_node_id,
                slug = excluded.slug,
                name = excluded.name,
                organisation_name = excluded.organisation_name,
                organisation_url = excluded.organisation_url,
                node_steward_conduct_url = excluded.node_steward_conduct_url,
                user_conduct_url = excluded.user_conduct_url,
                user_privacy_url = excluded.user_privacy_url",
            region.id,
            region.creator_node_id,
            region.slug,
            region.name,
            region.organisation_name,
            region.organisation_url,
            region.node_steward_conduct_url,
            region.user_conduct_url,
            region.user_privacy_url,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn upsert_id(
        &self,
        pool: &SqlitePool,
        region_id: &RegionId,
    ) -> Result<(), sqlx::Error> {
        let region_id_hex = region_id.to_hex();
        let _region = sqlx::query!(
            "INSERT INTO regions (id)
            VALUES (?)
            ON CONFLICT(id) DO NOTHING",
            region_id_hex,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn upsert_map(
        &self,
        pool: &SqlitePool,
        region_id: &RegionId,
        map_data_url: &Option<String>,
        min_latlng: &Option<LatLng>,
        max_latlng: &Option<LatLng>,
    ) -> Result<(), sqlx::Error> {
        let region_id_hex = region_id.to_hex();
        let min_latlng_json = min_latlng.clone().map(Json);
        let max_latlng_json = max_latlng.clone().map(Json);

        let _region = sqlx::query!(
            "UPDATE regions
            SET map = ?, min_latlng = ?, max_latlng = ?
            WHERE id = ?",
            map_data_url,
            min_latlng_json,
            max_latlng_json,
            region_id_hex,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
