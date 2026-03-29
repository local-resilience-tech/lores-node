use sqlx::SqlitePool;

use crate::{
    api::public_api::client_events::ClientEvent,
    data::{
        entities::{Region, RegionNodeStatus, RegionWithNodes},
        projections_read::region_nodes::RegionNodesReadRepo,
        projections_write::{region_nodes::RegionNodesWriteRepo, regions::RegionsWriteRepo},
    },
    event_handlers::utilities::{
        handle_db_write_error, header_has_region, EventHandler, HandlerResult,
    },
    panda_comms::lores_events::{LoResEventHeader, RegionCreatedDataV1},
};

pub struct RegionCreatedHandler {
    payload: RegionCreatedDataV1,
}

impl RegionCreatedHandler {
    pub fn new(payload: &RegionCreatedDataV1) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    async fn write_projections(
        &self,
        header: LoResEventHeader,
        pool: &SqlitePool,
    ) -> Result<RegionWithNodes, sqlx::Error> {
        let region_write_repo = RegionsWriteRepo::init();
        let node_write_repo = RegionNodesWriteRepo::init();
        let node_read_repo = RegionNodesReadRepo::init();

        let node_id = header.author_node_id;
        let region_id = match header.region_id.clone() {
            Some(id) => id,
            None => {
                println!("Error: header region ID is missing");
                return Err(sqlx::Error::ColumnNotFound("region_id".to_string()));
            }
        };

        let region = Region {
            id: region_id.to_hex(),
            creator_node_id: Some(node_id.clone()),
            slug: Some(self.payload.slug.clone()),
            name: Some(self.payload.name.clone()),
            organisation_name: self.payload.organisation_name.clone(),
            organisation_url: self.payload.organisation_url.clone(),
            node_steward_conduct_url: self.payload.node_steward_conduct_url.clone(),
            user_conduct_url: self.payload.user_conduct_url.clone(),
            user_privacy_url: self.payload.user_privacy_url.clone(),
            map: None,
        };
        region_write_repo.upsert(pool, &region).await?;

        // Upsert region node status
        node_write_repo
            .upsert_join_status_and_details(
                pool,
                &node_id,
                &region.id,
                RegionNodeStatus::Member,
                None,
                None,
                None,
            )
            .await?;

        let result = node_read_repo.append_detailed_nodes(pool, &region).await?;

        Ok(result)
    }
}

impl EventHandler for RegionCreatedHandler {
    async fn handle(&self, header: LoResEventHeader, pool: &SqlitePool) -> HandlerResult {
        let result = self.write_projections(header, pool).await;

        match result {
            Ok(region_with_nodes) => HandlerResult {
                client_events: vec![ClientEvent::NodeJoinedRegion(region_with_nodes)],
            },

            Err(e) => handle_db_write_error(e),
        }
    }

    async fn validate(&self, header: &LoResEventHeader, _pool: &SqlitePool) -> Result<(), ()> {
        header_has_region(header)
    }
}
