use sqlx::{Sqlite, SqlitePool};

use crate::{
    data::projections_write::region_nodes::RegionNodesWriteRepo,
    event_handlers::handler_utilities::{
        handle_db_write_error, node_id_is_author, read_node_updated_event, HandlerResult,
    },
    panda_comms::{
        lores_events::{LoResEventHeader, RegionNodeUpdatedDataV1},
        RegionId,
    },
};

pub struct RegionNodeUpdatedHandler {}

impl RegionNodeUpdatedHandler {
    pub async fn handle(
        header: LoResEventHeader,
        payload: RegionNodeUpdatedDataV1,
        pool: &sqlx::Pool<Sqlite>,
    ) -> HandlerResult {
        let region_id: RegionId = match RegionId::from_hex(&payload.region_id) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Invalid region ID in RegionNodeUpdated event: {}", e);
                return HandlerResult::default();
            }
        };

        println!(
            "Updating node {} in region {}",
            payload.node_id,
            region_id.to_hex()
        );

        if Self::validate(&header, &payload) {
            println!("Region node updated event validation passed");
        } else {
            println!("Region node updated event validation failed");
            return HandlerResult::default();
        }

        let result = Self::write_projections(&header, &payload, pool).await;

        match result {
            Ok(()) => HandlerResult {
                client_events: read_node_updated_event(pool, payload.node_id, region_id.to_hex())
                    .await,
            },

            Err(e) => handle_db_write_error(e),
        }
    }

    async fn write_projections(
        _header: &LoResEventHeader,
        payload: &RegionNodeUpdatedDataV1,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        let repo = RegionNodesWriteRepo::init();

        repo.upsert_details(pool, payload).await?;

        Ok(())
    }

    fn validate(header: &LoResEventHeader, payload: &RegionNodeUpdatedDataV1) -> bool {
        return node_id_is_author(&header, &payload.node_id);
    }
}
