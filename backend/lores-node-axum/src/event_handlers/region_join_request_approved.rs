use sqlx::SqlitePool;

use crate::{
    api::public_api::client_events::ClientEvent,
    data::{
        entities::{RegionNodeDetails, RegionNodeStatus},
        projections_read::{region_nodes::RegionNodesReadRepo, regions::RegionsReadRepo},
        projections_write::{region_nodes::RegionNodesWriteRepo, regions::RegionsWriteRepo},
    },
    event_handlers::utilities::{handle_db_write_error, EventHandler, HandlerResult},
    panda_comms::{
        lores_events::{LoResEventHeader, RegionJoinRequestApprovedDataV1},
        RegionId,
    },
};

pub struct RegionJoinRequestApprovedHandler {
    payload: RegionJoinRequestApprovedDataV1,
}

impl RegionJoinRequestApprovedHandler {
    pub fn new(payload: &RegionJoinRequestApprovedDataV1) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    async fn write_projections(
        &self,
        region_id: RegionId,
        pool: &SqlitePool,
    ) -> Result<RegionNodeDetails, sqlx::Error> {
        let regions_write_repo = RegionsWriteRepo::init();
        let node_write_repo = RegionNodesWriteRepo::init();
        let node_read_repo = RegionNodesReadRepo::init();

        // Ensure region exists
        regions_write_repo.upsert_id(pool, &region_id).await?;

        // Upsert region node status
        node_write_repo
            .upsert_join_status(
                pool,
                &self.payload.node_id,
                &region_id.to_hex(),
                RegionNodeStatus::Member,
            )
            .await?;

        // Get region node
        let region_node = match node_read_repo
            .find_detailed_by_keys(pool, self.payload.node_id.clone(), region_id.to_hex())
            .await?
        {
            Some(region_node) => region_node,
            None => {
                eprintln!("Region node not found after upsert: {}", region_id);
                return Err(sqlx::Error::RowNotFound);
            }
        };

        Ok(region_node)
    }

    async fn validate(&self, header: &LoResEventHeader, pool: &SqlitePool) -> bool {
        let region_id = match header.region_id.clone() {
            Some(id) => id,
            None => {
                println!("Validation failed: header region ID is missing");
                return false;
            }
        };

        if region_id.to_hex() != self.payload.region_id {
            println!(
                "Validation failed: payload region ID {:?} does not match header region ID {:?}",
                self.payload.region_id,
                region_id.to_hex()
            );
            return false;
        }

        // Get the region
        let repo = RegionsReadRepo::init();
        let region = match repo.find(pool, &region_id.to_hex()).await {
            Ok(Some(region)) => region,
            Ok(None) => {
                println!(
                    "Validation failed: region not found for ID {}",
                    region_id.to_hex()
                );
                return false;
            }
            Err(e) => {
                eprintln!("Database error during validation: {}", e);
                return false;
            }
        };

        let creator_node_id = match region.creator_node_id.clone() {
            Some(id) => id,
            None => {
                println!("Validation failed: region creator node ID is missing");
                return false;
            }
        };

        // The author node id should be the region creator
        if header.author_node_id != creator_node_id {
            println!(
                "Validation failed: author node ID {:?} does not match region creator node ID {:?}",
                header.author_node_id, creator_node_id
            );
            return false;
        }

        true
    }
}

impl EventHandler for RegionJoinRequestApprovedHandler {
    async fn handle(&self, header: LoResEventHeader, pool: &SqlitePool) -> HandlerResult {
        let region_id: RegionId = match RegionId::from_hex(&self.payload.region_id) {
            Ok(id) => id,
            Err(e) => {
                eprintln!(
                    "Invalid region ID in RegionJoinRequestApproved event: {}",
                    e
                );
                return HandlerResult::default();
            }
        };

        if self.validate(&header, pool).await {
            println!("Region join request approved event validation passed");
        } else {
            println!("Region join request approved event validation failed");
            return HandlerResult::default();
        }

        let result = self.write_projections(region_id, pool).await;

        match result {
            Ok(region_node) => HandlerResult {
                client_events: vec![ClientEvent::RegionNodeUpdated(region_node)],
            },

            Err(e) => handle_db_write_error(e),
        }
    }
}
