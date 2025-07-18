use self::node_announced::NodeAnnouncedHandler;
use sqlx::Sqlite;

use crate::{
    event_handlers::{
        handler_utilities::HandlerResult, node_status_posted::NodeStatusPostedHandler,
        node_updated::NodeUpdatedHandler,
    },
    panda_comms::lores_events::{LoResEvent, LoResEventPayload},
};

mod handler_utilities;
mod node_announced;
mod node_status_posted;
mod node_updated;

pub async fn handle_event(event: LoResEvent, pool: &sqlx::Pool<Sqlite>) {
    let header = event.header.clone();
    let payload = event.payload.clone();

    let result: HandlerResult = match payload {
        LoResEventPayload::NodeAnnounced(payload) => {
            NodeAnnouncedHandler::handle(header, payload, pool).await
        }
        LoResEventPayload::NodeUpdated(payload) => {
            NodeUpdatedHandler::handle(header, payload, pool).await
        }
        LoResEventPayload::NodeStatusPosted(payload) => {
            NodeStatusPostedHandler::handle(header, payload, pool).await
        }
    };

    if !result.client_events.is_empty() {
        // Here you would typically send the client events to the appropriate clients.
        // For example, using a WebSocket or similar mechanism.
        println!("Client events to send: {:?}", result.client_events);
    } else {
        println!("No client events to send.");
    }
}
