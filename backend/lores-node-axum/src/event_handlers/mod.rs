use sqlx::SqlitePool;

use crate::{
    api::public_api::realtime::RealtimeState,
    event_handlers::{
        app_registered::AppRegisteredHandler, handler_utilities::HandlerResult,
        node_status_posted::NodeStatusPostedHandler, region_created::RegionCreatedHandler,
        region_join_request_approved::RegionJoinRequestApprovedHandler,
        region_join_requested::RegionJoinRequestedHandler,
        region_node_updated::RegionNodeUpdatedHandler,
    },
    panda_comms::lores_events::{LoResEvent, LoResEventPayload},
};

mod app_registered;
mod handler_utilities;
mod node_status_posted;
mod region_created;
mod region_join_request_approved;
mod region_join_requested;
mod region_node_updated;

pub async fn handle_event(event: LoResEvent, pool: &SqlitePool, realtime_state: &RealtimeState) {
    let header = event.header.clone();
    let payload = event.payload.clone();

    let result: HandlerResult = match payload {
        LoResEventPayload::RegionCreated(payload) => {
            RegionCreatedHandler::handle(header, payload, pool).await
        }
        LoResEventPayload::RegionNodeUpdated(payload) => {
            RegionNodeUpdatedHandler::handle(header, payload, pool).await
        }
        LoResEventPayload::NodeStatusPosted(payload) => {
            NodeStatusPostedHandler::handle(header, payload, pool).await
        }
        LoResEventPayload::AppRegistered(payload) => {
            AppRegisteredHandler::handle(header, payload, pool).await
        }
        LoResEventPayload::RegionJoinRequested(payload) => {
            RegionJoinRequestedHandler::handle(header, payload, pool).await
        }
        LoResEventPayload::RegionJoinRequestApproved(payload) => {
            RegionJoinRequestApprovedHandler::handle(header, payload, pool).await
        }
        #[allow(unreachable_patterns)]
        _ => {
            eprintln!("Unhandled LoResEventPayload: {:?}", payload);
            HandlerResult::default()
        }
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
