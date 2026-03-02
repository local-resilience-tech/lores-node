use sqlx::{Sqlite, SqlitePool};

use crate::{
    data::projections_write::region_nodes::RegionNodesWriteRepo,
    event_handlers::utilities::{
        handle_db_write_error, node_id_is_author, read_node_updated_event, EventHandler,
        HandlerResult,
    },
    panda_comms::{
        lores_events::{LoResEventHeader, RegionNodeUpdatedDataV1},
        RegionId,
    },
};

pub struct RegionNodeUpdatedHandler {
    payload: RegionNodeUpdatedDataV1,
}

impl RegionNodeUpdatedHandler {
    pub fn new(payload: &RegionNodeUpdatedDataV1) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    async fn write_projections(
        &self,
        _header: &LoResEventHeader,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        let repo = RegionNodesWriteRepo::init();

        repo.upsert_details(pool, &self.payload).await?;

        Ok(())
    }

    fn validate(&self, header: &LoResEventHeader) -> bool {
        return node_id_is_author(&header, &self.payload.node_id);
    }
}

impl EventHandler for RegionNodeUpdatedHandler {
    async fn handle(&self, header: LoResEventHeader, pool: &sqlx::Pool<Sqlite>) -> HandlerResult {
        let region_id: RegionId = match RegionId::from_hex(&self.payload.region_id) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Invalid region ID in RegionNodeUpdated event: {}", e);
                return HandlerResult::default();
            }
        };

        if self.validate(&header) {
            println!("Region node updated event validation passed");
        } else {
            println!("Region node updated event validation failed");
            return HandlerResult::default();
        }

        let result = self.write_projections(&header, pool).await;

        match result {
            Ok(()) => HandlerResult {
                client_events: read_node_updated_event(
                    pool,
                    self.payload.node_id.clone(),
                    region_id.to_hex(),
                )
                .await,
            },

            Err(e) => handle_db_write_error(e),
        }
    }
}
