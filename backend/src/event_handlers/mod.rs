use self::node_announced::NodeAnnouncedHandler;
use sqlx::Sqlite;

use crate::{
    event_handlers::{
        node_status_posted::NodeStatusPostedHandler, node_updated::NodeUpdatedHandler,
    },
    panda_comms::lores_events::{LoResEvent, LoResEventPayload},
};

mod node_announced;
mod node_status_posted;
mod node_updated;

pub async fn handle_event(event: LoResEvent, pool: &sqlx::Pool<Sqlite>) {
    let header = event.header.clone();
    let payload = event.payload.clone();

    match payload {
        LoResEventPayload::NodeAnnounced(payload) => {
            NodeAnnouncedHandler::handle(header, payload, pool).await;
        }
        LoResEventPayload::NodeUpdated(payload) => {
            NodeUpdatedHandler::handle(header, payload, pool).await;
        }
        LoResEventPayload::NodeStatusPosted(payload) => {
            NodeStatusPostedHandler::handle(header, payload, pool).await;
        }
    }
}
