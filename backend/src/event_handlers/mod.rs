use self::node_announced::NodeAnnouncedHandler;
use sqlx::SqlitePool;

use crate::{
    event_handlers::{
        app_registered::AppRegisteredHandler, handler_utilities::HandlerResult,
        node_status_posted::NodeStatusPostedHandler, node_updated::NodeUpdatedHandler,
    },
    panda_comms::lores_events::{LoResEvent, LoResEventPayload},
    public_api::realtime::RealtimeState,
};

mod app_registered;
mod handler_utilities;
mod node_announced;
mod node_status_posted;
mod node_updated;

pub async fn handle_event(event: LoResEvent, pool: &SqlitePool, realtime_state: &RealtimeState) {
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
        LoResEventPayload::AppRegistered(payload) => {
            AppRegisteredHandler::handle(header, payload, pool).await
        } // _ => {
          //     eprintln!("Unhandled LoResEventPayload: {:?}", payload);
          //     HandlerResult::default()
          // }
    };

    if !result.client_events.is_empty() {
        // Here you would typically send the client events to the appropriate clients.
        // For example, using a WebSocket or similar mechanism.
        realtime_state
            .broadcast_app_events(result.client_events.clone())
            .await;
        println!("Client events to send: {:?}", result.client_events);
    } else {
        println!("No client events to send.");
    }
}
