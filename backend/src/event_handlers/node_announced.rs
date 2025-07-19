use sqlx::{Sqlite, SqlitePool};

use crate::{
    admin_api::client_events::ClientEvent,
    event_handlers::handler_utilities::HandlerResult,
    panda_comms::lores_events::{LoResEventHeader, NodeAnnouncedDataV1},
    projections::{
        entities::{Node, NodeDetails},
        projections_read::nodes::NodesReadRepo,
        projections_write::nodes::NodesWriteRepo,
    },
};

pub struct NodeAnnouncedHandler {}

impl NodeAnnouncedHandler {
    pub async fn handle(
        header: LoResEventHeader,
        payload: NodeAnnouncedDataV1,
        pool: &sqlx::Pool<Sqlite>,
    ) -> HandlerResult {
        println!("Node announced: {:?}", payload);

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
        payload: NodeAnnouncedDataV1,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        let repo = NodesWriteRepo::init();

        let node = Node {
            id: header.author_node_id.clone(),
            name: payload.name.clone(),
        };
        repo.upsert(pool, node).await?;
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
