use sqlx::SqlitePool;

use crate::{
    api::public_api::client_events::ClientEvent,
    data::projections_read::region_nodes::RegionNodesReadRepo,
    panda_comms::lores_events::LoResEventHeader,
};

#[derive(Default, Debug)]
pub struct HandlerResult {
    pub client_events: Vec<ClientEvent>,
}

pub fn handle_db_write_error(e: sqlx::Error) -> HandlerResult {
    eprintln!("Database write error: {}", e);
    HandlerResult::default()
}

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

pub fn node_id_is_author(header: &LoResEventHeader, node_id: &str) -> bool {
    if !header.author_node_id.is_empty() && header.author_node_id == node_id {
        true
    } else {
        println!(
            "Validation failed: author node ID {} does not match event node ID {}",
            header.author_node_id, node_id
        );
        false
    }
}
