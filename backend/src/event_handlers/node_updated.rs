use sqlx::Sqlite;

use crate::{
    admin_api::client_events::ClientEvent,
    event_handlers::handler_utilities::HandlerResult,
    panda_comms::lores_events::{LoResEventHeader, NodeUpdatedDataV1},
    projections::{
        entities::{Node, NodeDetails},
        projections_read::nodes::NodesReadRepo,
        projections_write::nodes::{NodeUpdateRow, NodesWriteRepo},
    },
};

pub struct NodeUpdatedHandler {}

impl NodeUpdatedHandler {
    pub async fn handle(
        header: LoResEventHeader,
        payload: NodeUpdatedDataV1,
        pool: &sqlx::Pool<Sqlite>,
    ) -> HandlerResult {
        let result = Self::write_projections(header, payload, pool).await;

        match result {
            Ok(Some(node_details)) => HandlerResult {
                client_events: vec![ClientEvent::NodeUpdated(node_details)],
            },
            Ok(None) => {
                println!("Node not found for update.");
                HandlerResult::default()
            }
            Err(e) => {
                eprintln!("Error updating node: {}", e);
                HandlerResult::default()
            }
        }
    }

    async fn write_projections(
        header: LoResEventHeader,
        payload: NodeUpdatedDataV1,
        pool: &sqlx::Pool<Sqlite>,
    ) -> Result<Option<NodeDetails>, sqlx::Error> {
        let repo = NodesWriteRepo::init();

        println!("Node updated: {:?}", payload);

        // Upsert the node for now. This wouldn't be needed if we had a preserved message log.
        let node = Node {
            id: header.author_node_id.clone(),
            name: payload.name.clone(),
        };
        repo.upsert(pool, node).await?;

        let node = NodeUpdateRow {
            id: header.author_node_id.clone(),
            name: payload.name.clone(),
            public_ipv4: Some(payload.public_ipv4.clone()),
        };

        repo.update(pool, node).await?;

        let read_repo = NodesReadRepo::init();

        read_repo
            .find_detailed(pool, header.author_node_id.clone())
            .await
    }
}
