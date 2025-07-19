use sqlx::Sqlite;

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

        let result = Self::write_projections(header, payload, pool).await;

        match result {
            Ok(Some(details)) => HandlerResult {
                client_events: vec![ClientEvent::NodeUpdated(details)],
            },
            Ok(None) => {
                println!("Node not found for announcement.");
                HandlerResult::default()
            }
            Err(e) => {
                eprintln!("Error handling node announcement: {}", e);
                HandlerResult::default()
            }
        }
    }

    pub async fn write_projections(
        header: LoResEventHeader,
        payload: NodeAnnouncedDataV1,
        pool: &sqlx::Pool<Sqlite>,
    ) -> Result<Option<NodeDetails>, sqlx::Error> {
        let repo = NodesWriteRepo::init();

        let node = Node {
            id: header.author_node_id.clone(),
            name: payload.name.clone(),
        };
        repo.upsert(pool, node).await?;

        let read_repo = NodesReadRepo::init();

        read_repo
            .find_detailed(pool, header.author_node_id.clone())
            .await
    }
}
