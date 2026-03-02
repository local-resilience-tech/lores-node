use crate::{
    api::public_api::client_events::ClientEvent, panda_comms::lores_events::LoResEventHeader,
};

pub use region_node_utils::read_node_updated_event;
use sqlx::SqlitePool;

pub mod null_handler;
mod region_node_utils;

pub trait EventHandler: Send + Sync {
    async fn validate(&self, header: &LoResEventHeader, _pool: &SqlitePool) -> Result<(), ()>;
    async fn handle(&self, header: LoResEventHeader, pool: &SqlitePool) -> HandlerResult;
}

#[derive(Default, Debug)]
pub struct HandlerResult {
    pub client_events: Vec<ClientEvent>,
}

pub fn handle_db_write_error(e: sqlx::Error) -> HandlerResult {
    eprintln!("Database write error: {}", e);
    HandlerResult::default()
}

pub fn node_id_is_author(header: &LoResEventHeader, node_id: &str) -> Result<(), ()> {
    if header.author_node_id.is_empty() || node_id.is_empty() {
        println!(
            "Validation failed: author node ID or event node ID is empty (author_node_id: {}, event node_id: {})",
            header.author_node_id, node_id
        );
    }

    if header.author_node_id != node_id {
        println!(
            "Validation failed: author node ID {} does not match event node ID {}",
            header.author_node_id, node_id
        );
        return Err(());
    }

    Ok(())
}
