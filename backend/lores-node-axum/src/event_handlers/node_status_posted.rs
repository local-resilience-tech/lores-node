use sqlx::SqlitePool;

use crate::{
    data::{
        projections_read::region_nodes::RegionNodesReadRepo,
        projections_write::{
            current_node_statuses::{CurrentNodeStatusRow, CurrentNodeStatusesWriteRepo},
            node_statuses::{NodeStatusRow, NodeStatusesWriteRepo},
            region_nodes::RegionNodesWriteRepo,
        },
    },
    event_handlers::utilities::{
        handle_db_write_error, header_has_region, read_node_updated_event, EventHandler,
        HandlerResult,
    },
    panda_comms::lores_events::{LoResEventHeader, NodeStatusPostedDataV1},
};

pub struct NodeStatusPostedHandler {
    payload: NodeStatusPostedDataV1,
}

impl NodeStatusPostedHandler {
    pub fn new(payload: &NodeStatusPostedDataV1) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    async fn write_projections(
        &self,
        header: &LoResEventHeader,
        region_id_string: &str,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        let region_nodes_read_repo = RegionNodesReadRepo::init();
        let region_nodes_write_repo = RegionNodesWriteRepo::init();
        let status_write_repo = NodeStatusesWriteRepo::init();
        let current_status_write_repo = CurrentNodeStatusesWriteRepo::init();

        region_nodes_write_repo
            .upsert_identity(pool, &header.author_node_id, region_id_string)
            .await?;

        let region_node = region_nodes_read_repo
            .find_required_by_keys(pool, &header.author_node_id, region_id_string)
            .await?;

        status_write_repo
            .upsert(
                pool,
                NodeStatusRow {
                    operation_id: header.operation_id.to_hex(),
                    author_node_id: header.author_node_id.clone(),
                    posted_timestamp: header.timestamp,
                    text: self.payload.text.clone(),
                    state: self.payload.state.clone(),
                },
            )
            .await?;

        current_status_write_repo
            .upsert(
                pool,
                CurrentNodeStatusRow {
                    region_node_id: region_node.id,
                    posted_timestamp: header.timestamp,
                    text: self.payload.text.clone(),
                    state: self.payload.state.clone(),
                },
            )
            .await?;

        Ok(())
    }
}

impl EventHandler for NodeStatusPostedHandler {
    async fn handle(&self, header: LoResEventHeader, pool: &SqlitePool) -> HandlerResult {
        let region_id_string = header.region_id.as_ref().unwrap().to_hex();
        let result = self
            .write_projections(&header, &region_id_string, pool)
            .await;

        match result {
            Ok(()) => HandlerResult {
                client_events: read_node_updated_event(
                    pool,
                    header.author_node_id,
                    region_id_string.clone(),
                )
                .await,
            },

            Err(e) => handle_db_write_error(e),
        }
    }

    async fn validate(&self, header: &LoResEventHeader, _pool: &SqlitePool) -> Result<(), ()> {
        header_has_region(header)
    }
}
