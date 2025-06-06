use sqlx::Sqlite;

use crate::{
    panda_comms::lores_events::{LoResEvent, LoResEventPayload},
    repos::{
        current_node_statuses::{CurrentNodeStatusRow, CurrentNodeStatusesRepo},
        entities::Node,
        node_statuses::{NodeStatusRow, NodeStatusesRepo},
        nodes::{NodeUpdateRow, NodesRepo},
    },
};

pub async fn handle_event(event: LoResEvent, pool: &sqlx::Pool<Sqlite>) {
    let header = event.header;

    match event.payload {
        LoResEventPayload::NodeAnnounced(payload) => {
            println!("Node announced: {:?}", payload);

            let node = Node {
                id: header.author_node_id.clone(),
                name: payload.name.clone(),
            };
            upsert_node(pool, node).await;
        }
        LoResEventPayload::NodeUpdated(payload) => {
            let repo = NodesRepo::init();

            println!("Node updated: {:?}", payload);

            // Upsert the node for now. This wouldn't be needed if we had a preserved message log.
            let node = Node {
                id: header.author_node_id.clone(),
                name: payload.name.clone(),
            };
            upsert_node(pool, node).await;

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
        LoResEventPayload::NodeStatusPosted(payload) => {
            let repo = NodeStatusesRepo::init();

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

            let repo = CurrentNodeStatusesRepo::init();

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
        }
    }
}

async fn upsert_node(pool: &sqlx::Pool<Sqlite>, node: Node) {
    let repo = NodesRepo::init();

    repo.upsert(pool, node).await.unwrap();
}
