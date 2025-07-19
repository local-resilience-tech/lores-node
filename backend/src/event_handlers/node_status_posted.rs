use sqlx::{Sqlite, SqlitePool};

use crate::{
    admin_api::client_events::ClientEvent,
    event_handlers::handler_utilities::HandlerResult,
    panda_comms::lores_events::{LoResEventHeader, NodeStatusPostedDataV1},
    projections::{
        entities::NodeDetails,
        projections_read::nodes::NodesReadRepo,
        projections_write::{
            current_node_statuses::{CurrentNodeStatusRow, CurrentNodeStatusesWriteRepo},
            node_statuses::{NodeStatusRow, NodeStatusesWriteRepo},
        },
    },
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
            Ok(()) => {
                let client_events = Self::client_events(pool, author_node_id).await;
                HandlerResult { client_events }
            }

            Err(e) => {
                eprintln!("Error handling node announcement: {}", e);
                HandlerResult::default()
            }
        }
    }

    async fn client_events(pool: &SqlitePool, node_id: String) -> Vec<ClientEvent> {
        let node_details = Self::read_projections(pool, node_id).await;
        match node_details {
            Ok(Some(details)) => vec![ClientEvent::NodeUpdated(details)],
            Ok(None) => {
                println!("Node not found for announcement.");
                vec![]
            }
            Err(e) => {
                eprintln!("Error reading node details: {}", e);
                vec![]
            }
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

    async fn read_projections(
        pool: &SqlitePool,
        node_id: String,
    ) -> Result<Option<NodeDetails>, sqlx::Error> {
        let read_repo = NodesReadRepo::init();
        read_repo.find_detailed(pool, node_id).await
    }
}
