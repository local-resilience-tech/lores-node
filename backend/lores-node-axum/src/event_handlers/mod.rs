use sqlx::SqlitePool;

use crate::{
    api::public_api::realtime::RealtimeState,
    event_handlers::{
        app_registered::AppRegisteredHandler,
        node_status_posted::NodeStatusPostedHandler,
        region_created::RegionCreatedHandler,
        region_join_request_approved::RegionJoinRequestApprovedHandler,
        region_join_requested::RegionJoinRequestedHandler,
        region_node_updated::RegionNodeUpdatedHandler,
        utilities::{EventHandler, HandlerResult},
    },
    panda_comms::lores_events::{LoResEvent, LoResEventHeader, LoResEventPayload},
};

mod app_registered;
mod node_status_posted;
mod region_created;
mod region_join_request_approved;
mod region_join_requested;
mod region_node_updated;
mod utilities;

pub async fn handle_event(event: LoResEvent, pool: &SqlitePool, realtime_state: &RealtimeState) {
    let header = event.header.clone();
    let payload = event.payload.clone();

    let handler = get_handler(&payload);

    if let Err(e) = handler.validate(&header, &pool).await {
        eprintln!("This event is not valid: {:?}", e);
        return;
    }

    let handle_result = handler.handle(header, pool).await;

    if !handle_result.client_events.is_empty() {
        // Here you would typically send the client events to the appropriate clients.
        // For example, using a WebSocket or similar mechanism.
        realtime_state
            .broadcast_app_events(handle_result.client_events.clone())
            .await;
        println!("Client events to send: {:?}", handle_result.client_events);
    } else {
        println!("No client events to send.");
    }
}

macro_rules! define_handlers {
    ($($variant:ident => $handler:ty),+ $(,)?) => {
        enum EventHandlerBox {
            $($variant($handler)),+
        }

        impl EventHandler for EventHandlerBox {
            async fn handle(&self, header: LoResEventHeader, pool: &SqlitePool) -> HandlerResult {
                match self {
                    $(EventHandlerBox::$variant(h) => h.handle(header, pool).await),+
                }
            }

            async fn validate(&self, header: &LoResEventHeader, pool: &SqlitePool) -> Result<(), ()> {
                match self {
                    $(EventHandlerBox::$variant(h) => h.validate(header, pool).await),+
                }
            }
        }

        fn get_handler(payload: &LoResEventPayload) -> EventHandlerBox {
            match payload {
                $(LoResEventPayload::$variant(data) => {
                    EventHandlerBox::$variant(<$handler>::new(data))
                }),+
            }
        }
    };
}

// Single place to add new handlers!
define_handlers!(
    RegionCreated => RegionCreatedHandler,
    RegionNodeUpdated => RegionNodeUpdatedHandler,
    NodeStatusPosted => NodeStatusPostedHandler,
    AppRegistered => AppRegisteredHandler,
    RegionJoinRequested => RegionJoinRequestedHandler,
    RegionJoinRequestApproved => RegionJoinRequestApprovedHandler
);
