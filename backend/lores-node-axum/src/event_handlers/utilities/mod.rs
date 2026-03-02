use crate::{
    api::public_api::client_events::ClientEvent, panda_comms::lores_events::LoResEventHeader,
};

pub use region_node_utils::read_node_updated_event;
pub use region_utils::header_has_region;
use sqlx::SqlitePool;

pub mod null_handler;
mod region_node_utils;
pub mod region_utils;

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
