use sqlx::Sqlite;

use crate::{
    panda_comms::lores_events::{LoResEvent, LoResEventPayload},
    repos::{
        entities::{Node, NodeDetails, NodeStatus},
        node_statuses::NodeStatusesRepo,
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

            let result = repo.update(pool, node).await;

            if let Err(e) = result {
                println!("Error updating node: {}", e);
            } else {
                println!("Node updated successfully");
            }
        }
        LoResEventPayload::NodeStatusPosted(payload) => {
            let repo = NodeStatusesRepo::init();

            println!("Node status posted: {:?}", payload);

            let result = repo
                .upsert(
                    pool,
                    NodeStatus {
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
        }
    }
}
