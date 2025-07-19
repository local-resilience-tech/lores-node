use sqlx::Sqlite;

use crate::{
    event_handlers::handler_utilities::HandlerResult,
    panda_comms::lores_events::{LoResEventHeader, NodeAnnouncedDataV1},
    projections::{entities::Node, projections_write::nodes::NodesWriteRepo},
};

pub struct NodeAnnouncedHandler {}

impl NodeAnnouncedHandler {
    pub async fn handle(
        header: LoResEventHeader,
        payload: NodeAnnouncedDataV1,
        pool: &sqlx::Pool<Sqlite>,
    ) -> HandlerResult {
        println!("Node announced: {:?}", payload);

        let node = Node {
            id: header.author_node_id.clone(),
            name: payload.name.clone(),
        };
        NodesWriteRepo::init().upsert(pool, node).await.unwrap();

        HandlerResult::default()
    }
}
