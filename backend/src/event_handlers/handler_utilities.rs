use sqlx::SqlitePool;

use crate::{
    api::public_api::client_events::ClientEvent, data::projections_read::nodes::NodesReadRepo,
};

#[derive(Default, Debug)]
pub struct HandlerResult {
    pub client_events: Vec<ClientEvent>,
}

pub fn handle_db_write_error(e: sqlx::Error) -> HandlerResult {
    eprintln!("Database write error: {}", e);
    HandlerResult::default()
}

pub async fn read_node_updated_event(pool: &SqlitePool, node_id: String) -> Vec<ClientEvent> {
    let node_details = NodesReadRepo::init().find_detailed(pool, node_id).await;
    match node_details {
        Ok(Some(details)) => vec![ClientEvent::NodeUpdated(details)],
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
