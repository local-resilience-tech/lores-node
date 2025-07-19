use sqlx::Sqlite;

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
        let result = Self::write_projections(header, payload, pool).await;
        match result {
            Ok(Some(details)) => HandlerResult {
                client_events: vec![ClientEvent::NodeUpdated(details)],
            },
            Ok(None) => {
                println!("Node not found for status posting.");
                HandlerResult::default()
            }
            Err(e) => {
                eprintln!("Error posting node status: {}", e);
                HandlerResult::default()
            }
        }
    }

    pub async fn write_projections(
        header: LoResEventHeader,
        payload: NodeStatusPostedDataV1,
        pool: &sqlx::Pool<Sqlite>,
    ) -> Result<Option<NodeDetails>, sqlx::Error> {
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

        let read_repo = NodesReadRepo::init();

        read_repo
            .find_detailed(pool, header.author_node_id.clone())
            .await
    }
}
