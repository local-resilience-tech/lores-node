use sqlx::SqlitePool;

use crate::{
    api::public_api::client_events::ClientEvent,
    data::{entities::Region, projections_write::regions::RegionsWriteRepo},
    event_handlers::handler_utilities::{handle_db_write_error, HandlerResult},
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
            Ok(region) => HandlerResult {
                client_events: vec![ClientEvent::JoinedRegion(region)],
            },

            Err(e) => handle_db_write_error(e),
        }
    }

    async fn write_projections(
        header: LoResEventHeader,
        payload: RegionCreatedDataV1,
        pool: &SqlitePool,
    ) -> Result<Region, sqlx::Error> {
        let repo = RegionsWriteRepo::init();

        let region = Region {
            id: payload.region_id,
            creator_node_id: header.author_node_id,
            slug: payload.slug,
            name: payload.name,
            organisation_name: payload.organisation_name,
            organisation_url: payload.organisation_url,
            node_steward_conduct_url: payload.node_steward_conduct_url,
            user_conduct_url: payload.user_conduct_url,
            user_privacy_url: payload.user_privacy_url,
        };

        repo.insert(pool, &region).await?;
        Ok(region)
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
