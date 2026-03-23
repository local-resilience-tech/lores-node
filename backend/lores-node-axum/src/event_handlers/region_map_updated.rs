use sqlx::SqlitePool;

use crate::{
    api::public_api::client_events::ClientEvent,
    data::{
        entities::{Region, RegionMap},
        projections_read::regions::RegionsReadRepo,
        projections_write::regions::RegionsWriteRepo,
    },
    event_handlers::utilities::{
        handle_db_write_error, header_has_region, EventHandler, HandlerResult,
    },
    panda_comms::{
        lores_events::{LoResEventHeader, RegionMapUpdatedDataV1},
        RegionId,
    },
};

pub struct RegionMapUpdatedHandler {
    payload: RegionMapUpdatedDataV1,
}

impl RegionMapUpdatedHandler {
    pub fn new(payload: &RegionMapUpdatedDataV1) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    async fn write_projections(
        &self,
        region_id: RegionId,
        pool: &SqlitePool,
    ) -> Result<Region, sqlx::Error> {
        let regions_write_repo = RegionsWriteRepo::init();
        let regions_read_repo = RegionsReadRepo::init();

        // Ensure region exists
        regions_write_repo
            .upsert_map(
                pool,
                &region_id,
                Some(RegionMap {
                    map_data_url: self.payload.image_data_url.clone(),
                    min_latlng: self.payload.min_latlng.clone(),
                    max_latlng: self.payload.max_latlng.clone(),
                }),
            )
            .await?;

        // Get region
        let region = match regions_read_repo.find(pool, &region_id.to_hex()).await? {
            Some(region) => region,
            None => {
                eprintln!("Region not found after upsert: {}", region_id);
                return Err(sqlx::Error::RowNotFound);
            }
        };

        Ok(region)
    }
}

impl EventHandler for RegionMapUpdatedHandler {
    async fn handle(&self, header: LoResEventHeader, pool: &SqlitePool) -> HandlerResult {
        let region_id = header.region_id.clone().unwrap();

        let result = self.write_projections(region_id, pool).await;

        match result {
            Ok(region) => HandlerResult {
                client_events: vec![ClientEvent::RegionUpdated(region)],
            },

            Err(e) => handle_db_write_error(e),
        }
    }

    async fn validate(&self, header: &LoResEventHeader, pool: &SqlitePool) -> Result<(), ()> {
        header_has_region(header)?;

        let region_id = header.region_id.clone().unwrap();

        // Get the region
        let repo = RegionsReadRepo::init();
        let region = match repo.find(pool, &region_id.to_hex()).await {
            Ok(Some(region)) => region,
            Ok(None) => {
                println!(
                    "Validation failed: region not found for ID {}",
                    region_id.to_hex()
                );
                return Err(());
            }
            Err(e) => {
                eprintln!("Database error during validation: {}", e);
                return Err(());
            }
        };

        let creator_node_id = match region.creator_node_id.clone() {
            Some(id) => id,
            None => {
                println!("Validation failed: region creator node ID is missing");
                return Err(());
            }
        };

        // The author node id should be the region creator
        if header.author_node_id != creator_node_id {
            println!(
                "Validation failed: author node ID {:?} does not match region creator node ID {:?}",
                header.author_node_id, creator_node_id
            );
            return Err(());
        }

        Ok(())
    }
}
