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
        handle_db_write_error, node_id_is_author, read_node_updated_event, EventHandler,
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
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        let region_nodes_read_repo = RegionNodesReadRepo::init();
        let region_nodes_write_repo = RegionNodesWriteRepo::init();
        let status_write_repo = NodeStatusesWriteRepo::init();
        let current_status_write_repo = CurrentNodeStatusesWriteRepo::init();

        region_nodes_write_repo
            .upsert_identity(pool, &self.payload.node_id, &self.payload.region_id)
            .await?;

        let region_node = region_nodes_read_repo
            .find_required_by_keys(pool, &self.payload.node_id, &self.payload.region_id)
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
        let result = self.write_projections(&header, pool).await;

        match result {
            Ok(()) => HandlerResult {
                client_events: read_node_updated_event(
                    pool,
                    header.author_node_id,
                    self.payload.region_id.clone(),
                )
                .await,
            },

            Err(e) => handle_db_write_error(e),
        }
    }

    async fn validate(&self, header: &LoResEventHeader, _pool: &SqlitePool) -> Result<(), ()> {
        // Ensure has region
        if self.payload.region_id.is_empty() {
            eprintln!("Validation failed: region_id or node_id is empty");
            return Err(());
        }

        return node_id_is_author(&header, &self.payload.node_id);
    }
}
