use sqlx::{Sqlite, SqlitePool};

use crate::{
    data::projections_write::{
        current_node_statuses::{CurrentNodeStatusRow, CurrentNodeStatusesWriteRepo},
        node_statuses::{NodeStatusRow, NodeStatusesWriteRepo},
    },
    event_handlers::handler_utilities::{
        handle_db_write_error, read_node_updated_event, HandlerResult,
    },
    panda_comms::lores_events::{LoResEventHeader, NodeStatusPostedDataV1},
};

pub struct NodeStatusPostedHandler {}

impl NodeStatusPostedHandler {
    pub async fn handle(
        header: LoResEventHeader,
        payload: NodeStatusPostedDataV1,
        pool: &sqlx::Pool<Sqlite>,
    ) -> HandlerResult {
        let author_node_id = header.author_node_id.clone();
        let result = Self::write_projections(header, payload, pool).await;

        match result {
            Ok(()) => HandlerResult {
                client_events: read_node_updated_event(pool, author_node_id).await,
            },

            Err(e) => handle_db_write_error(e),
        }
    }

    async fn write_projections(
        header: LoResEventHeader,
        payload: NodeStatusPostedDataV1,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        let repo = NodeStatusesWriteRepo::init();

        println!("Node status posted: {:?}", payload);

        let result = repo
            .upsert(
                pool,
                NodeStatusRow {
                    operation_id: header.operation_id.to_hex(),
                    author_node_id: header.author_node_id.clone(),
                    posted_timestamp: header.timestamp,
                    text: payload.text.clone(),
                    state: payload.state.clone(),
                },
            )
            .await;

        if let Err(e) = result {
            println!("Error posting node status: {}", e);
        } else {
            println!("Node status posted successfully");
        }

        let repo = CurrentNodeStatusesWriteRepo::init();

        let result = repo
            .upsert(
                pool,
                CurrentNodeStatusRow {
                    author_node_id: header.author_node_id.clone(),
                    posted_timestamp: header.timestamp,
                    text: payload.text.clone(),
                    state: payload.state.clone(),
                },
            )
            .await;

        if let Err(e) = result {
            println!("Error posting current node status: {}", e);
        } else {
            println!("Current node status posted successfully");
        }

        Ok(())
    }
}
