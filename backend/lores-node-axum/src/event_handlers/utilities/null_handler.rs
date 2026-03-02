use sqlx::SqlitePool;

use crate::{
    event_handlers::utilities::{EventHandler, HandlerResult},
    panda_comms::lores_events::LoResEventHeader,
};

// Keep this for if we need it when we don't have a handler yet for new types
#[allow(dead_code)]
pub struct NullHandler;

impl NullHandler {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

impl EventHandler for NullHandler {
    async fn handle(&self, _header: LoResEventHeader, _pool: &SqlitePool) -> HandlerResult {
        println!("NullHandler invoked - no operation performed");
        HandlerResult::default()
    }
}
