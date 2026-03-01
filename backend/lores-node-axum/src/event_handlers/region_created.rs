use sqlx::SqlitePool;

use crate::{
    api::public_api::client_events::ClientEvent,
    data::{
        entities::{Region, RegionNodeStatus, RegionWithNodes},
        projections_read::region_nodes::RegionNodesReadRepo,
        projections_write::{region_nodes::RegionNodesWriteRepo, regions::RegionsWriteRepo},
    },
    event_handlers::utilities::{handle_db_write_error, HandlerResult},
    panda_comms::lores_events::{LoResEventHeader, RegionCreatedDataV1},
};

pub struct RegionCreatedHandler {}

impl RegionCreatedHandler {
    pub async fn handle(
        header: LoResEventHeader,
        payload: RegionCreatedDataV1,
        pool: &SqlitePool,
    ) -> HandlerResult {
        println!("Region created: {:?}", payload);

        if Self::validate(&header, &payload) {
            println!("Region created event validation passed");
        } else {
            println!("Region created event validation failed");
            return HandlerResult::default();
        }

        let result = Self::write_projections(header, payload, pool).await;

        match result {
            Ok(region_with_nodes) => HandlerResult {
                client_events: vec![ClientEvent::JoinedRegion(region_with_nodes)],
            },

            Err(e) => handle_db_write_error(e),
        }
    }

    async fn write_projections(
        header: LoResEventHeader,
        payload: RegionCreatedDataV1,
        pool: &SqlitePool,
    ) -> Result<RegionWithNodes, sqlx::Error> {
        let region_write_repo = RegionsWriteRepo::init();
        let node_write_repo = RegionNodesWriteRepo::init();
        let node_read_repo = RegionNodesReadRepo::init();

        let node_id = header.author_node_id;

        let region = Region {
            id: payload.region_id,
            creator_node_id: Some(node_id.clone()),
            slug: Some(payload.slug),
            name: Some(payload.name),
            organisation_name: payload.organisation_name,
            organisation_url: payload.organisation_url,
            node_steward_conduct_url: payload.node_steward_conduct_url,
            user_conduct_url: payload.user_conduct_url,
            user_privacy_url: payload.user_privacy_url,
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

    fn validate(header: &LoResEventHeader, payload: &RegionCreatedDataV1) -> bool {
        let region_id = match header.region_id.clone() {
            Some(id) => id,
            None => {
                println!("Validation failed: header region ID is missing");
                return false;
            }
        };

        if region_id.to_hex() != payload.region_id {
            println!(
                "Validation failed: payload region ID {:?} does not match header region ID {:?}",
                payload.region_id,
                region_id.to_hex()
            );
            return false;
        }

        true
    }
}
