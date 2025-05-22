use sqlx::Sqlite;

use crate::{
    panda_comms::lores_events::{LoResEvent, LoResEventPayload},
    repos::{
        entities::{Node, NodeDetails},
        nodes::NodesRepo,
    },
};

pub async fn handle_event(event: LoResEvent, pool: &sqlx::Pool<Sqlite>) {
    let header = event.header;

    match event.payload {
        LoResEventPayload::NodeAnnounced(payload) => {
            let repo = NodesRepo::init();

            println!("Node announced: {:?}", payload);

            let node = Node {
                id: header.author_node_id.clone(),
                name: payload.name.clone(),
            };

            repo.upsert(pool, node).await.unwrap();
        }
        LoResEventPayload::NodeUpdated(payload) => {
            let repo = NodesRepo::init();

            println!("Node updated: {:?}", payload);

            let node = NodeDetails {
                id: header.author_node_id.clone(),
                name: payload.name.clone(),
                public_ipv4: Some(payload.public_ipv4.clone()),
            };

            repo.update(pool, node).await.unwrap();
        }
    }
}
