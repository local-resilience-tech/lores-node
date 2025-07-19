use sqlx::Sqlite;

use crate::{
    panda_comms::lores_events::{LoResEventHeader, NodeStatusPostedDataV1},
    projections::projections_write::{
        current_node_statuses::{CurrentNodeStatusRow, CurrentNodeStatusesWriteRepo},
        node_statuses::{NodeStatusRow, NodeStatusesWriteRepo},
    },
};

pub struct NodeStatusPostedHandler {}

impl NodeStatusPostedHandler {
    pub async fn handle(
        header: LoResEventHeader,
        payload: NodeStatusPostedDataV1,
        pool: &sqlx::Pool<Sqlite>,
    ) {
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
    }
}
