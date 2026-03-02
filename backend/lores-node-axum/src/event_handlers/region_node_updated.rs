use sqlx::{Sqlite, SqlitePool};

use crate::{
    data::projections_write::region_nodes::RegionNodesWriteRepo,
    event_handlers::utilities::{
        handle_db_write_error, read_node_updated_event, EventHandler, HandlerResult,
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
        header: &LoResEventHeader,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        let repo = RegionNodesWriteRepo::init();

        repo.upsert_details(
            pool,
            &header.region_id.as_ref().unwrap().to_hex(),
            &header.author_node_id,
            &self.payload,
        )
        .await?;

        Ok(())
    }
}

impl EventHandler for RegionNodeUpdatedHandler {
    async fn handle(&self, header: LoResEventHeader, pool: &sqlx::Pool<Sqlite>) -> HandlerResult {
        let region_id: RegionId = match header.region_id.clone() {
            Some(id) => id,
            None => {
                eprintln!("Region ID is missing");
                return HandlerResult::default();
            }
        };
        let node_id = header.author_node_id.clone();

        let result = self.write_projections(&header, pool).await;

        match result {
            Ok(()) => HandlerResult {
                client_events: read_node_updated_event(pool, node_id, region_id.to_hex()).await,
            },

            Err(e) => handle_db_write_error(e),
        }
    }

    async fn validate(&self, _header: &LoResEventHeader, _pool: &SqlitePool) -> Result<(), ()> {
        Ok(())
    }
}
