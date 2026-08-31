use sqlx::SqlitePool;

use crate::{
    api::public_api::client_events::ClientEvent,
    data::entities::NodeHeartbeat,
    event_handlers::utilities::{EventHandler, HandlerResult, header_has_region},
    panda_comms::lores_events::{LoResEventHeader, NodeHeartbeatDataV1},
};

pub struct NodeHeartbeatHandler {
    payload: NodeHeartbeatDataV1,
}

impl NodeHeartbeatHandler {
    pub fn new(payload: &NodeHeartbeatDataV1) -> Self {
        Self {
            payload: payload.clone(),
        }
    }
}

impl EventHandler for NodeHeartbeatHandler {
    async fn handle(&self, header: LoResEventHeader, _: &SqlitePool) -> HandlerResult {
        let node_heartbeat = NodeHeartbeat {
            region_id: header.region_id.clone().unwrap().to_string(),
            node_id: header.author_node_id.clone(),
        };

        HandlerResult {
            client_events: vec![ClientEvent::NodeHeartbeatReceived(node_heartbeat)],
        }
    }

    async fn validate(&self, header: &LoResEventHeader, _: &SqlitePool) -> Result<(), ()> {
        header_has_region(header)
    }
}
