use sqlx::SqlitePool;

use crate::{data::entities::Region, panda_comms::RegionId};

pub struct RegionsWriteRepo {}

impl RegionsWriteRepo {
    pub fn init() -> Self {
        RegionsWriteRepo {}
    }

    pub async fn insert(&self, pool: &SqlitePool, region: &Region) -> Result<(), sqlx::Error> {
        let _region = sqlx::query!(
            "INSERT INTO regions (
                id, creator_node_id, slug, name, organisation_name,
                organisation_url, node_steward_conduct_url, user_conduct_url, user_privacy_url
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
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
}
