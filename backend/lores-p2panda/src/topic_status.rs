use std::collections::HashMap;

use p2panda::streams::StreamEvent;
use p2panda_core::VerifyingKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionStatus {
    Unknown,
    Syncing,
}

/// Tracks the known connection status of remote nodes for a single topic subscription.
#[derive(Debug, Default)]
pub struct TopicStatus {
    connections: HashMap<VerifyingKey, ConnectionStatus>,
}

impl TopicStatus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_stream_event<M>(&mut self, event: &StreamEvent<M>) {
        match event {
            StreamEvent::SyncStarted { remote_node_id, .. } => {
                self.connections
                    .insert(*remote_node_id, ConnectionStatus::Syncing);
            }
            StreamEvent::SyncEnded { remote_node_id, .. } => {
                self.connections
                    .entry(*remote_node_id)
                    .and_modify(|s| *s = ConnectionStatus::Unknown);
            }
            _ => {}
        }
    }

    pub fn connections(&self) -> &HashMap<VerifyingKey, ConnectionStatus> {
        &self.connections
    }
}
