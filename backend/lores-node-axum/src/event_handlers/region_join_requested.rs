use sqlx::SqlitePool;

use crate::{
    api::public_api::client_events::ClientEvent,
    data::{
        entities::{Region, RegionNodeStatus},
        projections_read::regions::RegionsReadRepo,
        projections_write::{region_nodes::RegionNodesWriteRepo, regions::RegionsWriteRepo},
    },
    event_handlers::handler_utilities::{handle_db_write_error, HandlerResult},
    panda_comms::{
        lores_events::{LoResEventHeader, RegionJoinRequestedDataV1},
        RegionId,
    },
};

pub struct RegionJoinRequestedHandler {}

impl RegionJoinRequestedHandler {
    pub async fn handle(
        header: LoResEventHeader,
        payload: RegionJoinRequestedDataV1,
        pool: &SqlitePool,
    ) -> HandlerResult {
        println!("Region join requested: {:?}", payload);

        let region_id: RegionId = match RegionId::from_hex(&payload.region_id) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Invalid region ID in RegionJoinRequested event: {}", e);
                return HandlerResult::default();
            }
        };

        let result = Self::write_projections(header, payload, region_id, pool).await;

        match result {
            Ok(region) => HandlerResult {
                client_events: vec![ClientEvent::JoinedRegion(region)],
            },

            Err(e) => handle_db_write_error(e),
        }
    }

    async fn write_projections(
        header: LoResEventHeader,
        payload: RegionJoinRequestedDataV1,
        region_id: RegionId,
        pool: &SqlitePool,
    ) -> Result<Region, sqlx::Error> {
        let regions_write_repo = RegionsWriteRepo::init();
        let regions_read_repo = RegionsReadRepo::init();
        let node_repo = RegionNodesWriteRepo::init();

        let node_id = header.author_node_id;

        // Ensure region exists
        regions_write_repo.upsert_id(pool, &region_id).await?;

        // Get region
        let region = match regions_read_repo.find(pool, &region_id.to_hex()).await? {
            Some(region) => region,
            None => {
                eprintln!("Region not found after upsert: {}", region_id);
                return Err(sqlx::Error::RowNotFound);
            }
        };

        // Upsert region node status
        node_repo
            .upsert_join_status(
                pool,
                &node_id,
                &region_id.to_hex(),
                RegionNodeStatus::RequestedToJoin,
                Some(payload.about_your_node),
                Some(payload.about_your_stewards),
                payload.agreed_node_steward_conduct_url,
            )
            .await?;

        Ok(region)
    }
}
