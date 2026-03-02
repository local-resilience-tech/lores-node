use sqlx::SqlitePool;

use crate::{
    api::public_api::client_events::ClientEvent,
    data::projections_read::region_nodes::RegionNodesReadRepo,
};

pub async fn read_node_updated_event(
    pool: &SqlitePool,
    node_id: String,
    region_id: String,
) -> Vec<ClientEvent> {
    let node_details = RegionNodesReadRepo::init()
        .find_detailed_by_keys(pool, node_id, region_id)
        .await;
    match node_details {
        Ok(Some(details)) => vec![ClientEvent::RegionNodeUpdated(details)],
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
