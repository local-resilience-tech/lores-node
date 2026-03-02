use sqlx::SqlitePool;

use crate::{
    api::public_api::client_events::ClientEvent,
    data::{
        entities::{RegionNodeStatus, RegionWithNodes},
        projections_read::{region_nodes::RegionNodesReadRepo, regions::RegionsReadRepo},
        projections_write::{region_nodes::RegionNodesWriteRepo, regions::RegionsWriteRepo},
    },
    event_handlers::utilities::{handle_db_write_error, EventHandler, HandlerResult},
    panda_comms::{
        lores_events::{LoResEventHeader, RegionJoinRequestedDataV1},
        RegionId,
    },
};

pub struct RegionJoinRequestedHandler {
    payload: RegionJoinRequestedDataV1,
}

impl RegionJoinRequestedHandler {
    pub fn new(payload: &RegionJoinRequestedDataV1) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    async fn write_projections(
        &self,
        header: LoResEventHeader,
        region_id: RegionId,
        pool: &SqlitePool,
    ) -> Result<RegionWithNodes, sqlx::Error> {
        let regions_write_repo = RegionsWriteRepo::init();
        let regions_read_repo = RegionsReadRepo::init();
        let node_write_repo = RegionNodesWriteRepo::init();
        let node_read_repo = RegionNodesReadRepo::init();

        let node_id = header.author_node_id;

        // Ensure region exists
        regions_write_repo.upsert_id(pool, &region_id).await?;

        // Upsert region node status
        node_write_repo
            .upsert_join_status_and_details(
                pool,
                &node_id,
                &region_id.to_hex(),
                RegionNodeStatus::RequestedToJoin,
                Some(self.payload.about_your_node.clone()),
                Some(self.payload.about_your_stewards.clone()),
                self.payload.agreed_node_steward_conduct_url.clone(),
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
        let result = node_read_repo.append_detailed_nodes(pool, &region).await?;

        Ok(result)
    }
}

impl EventHandler for RegionJoinRequestedHandler {
    async fn handle(&self, header: LoResEventHeader, pool: &SqlitePool) -> HandlerResult {
        println!("Region join requested: {:?}", self.payload);

        let region_id: RegionId = match RegionId::from_hex(&self.payload.region_id) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Invalid region ID in RegionJoinRequested event: {}", e);
                return HandlerResult::default();
            }
        };

        let result = self.write_projections(header, region_id, pool).await;

        match result {
            Ok(region_with_nodes) => HandlerResult {
                client_events: vec![ClientEvent::JoinedRegion(region_with_nodes)],
            },

            Err(e) => handle_db_write_error(e),
        }
    }

    async fn validate(&self, _header: &LoResEventHeader, _pool: &SqlitePool) -> Result<(), ()> {
        Ok(())
    }
}
