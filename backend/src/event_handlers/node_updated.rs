use sqlx::Sqlite;

use crate::{
    panda_comms::lores_events::{LoResEventHeader, NodeUpdatedDataV1},
    projections::{
        entities::Node,
        projections_write::nodes::{NodeUpdateRow, NodesWriteRepo},
    },
};

pub struct NodeUpdatedHandler {}

impl NodeUpdatedHandler {
    pub async fn handle(
        header: LoResEventHeader,
        payload: NodeUpdatedDataV1,
        pool: &sqlx::Pool<Sqlite>,
    ) {
        let repo = NodesWriteRepo::init();

        println!("Node updated: {:?}", payload);

        // Upsert the node for now. This wouldn't be needed if we had a preserved message log.
        let node = Node {
            id: header.author_node_id.clone(),
            name: payload.name.clone(),
        };
        NodesWriteRepo::init().upsert(pool, node).await.unwrap();

        let node = NodeUpdateRow {
            id: header.author_node_id.clone(),
            name: payload.name.clone(),
            public_ipv4: Some(payload.public_ipv4.clone()),
        };

        let result = repo.update(pool, node).await;

        if let Err(e) = result {
            println!("Error updating node: {}", e);
        } else {
            println!("Node updated successfully");
        }
    }
}
