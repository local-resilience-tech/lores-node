use std::collections::HashMap;

use p2panda::streams::StreamEvent;
use p2panda_core::VerifyingKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionStatus {
    /// Peer has been observed but no sync session has completed yet.
    Unknown,
    /// A sync session with this peer is currently in progress.
    Syncing,
    /// The last sync session with this peer completed successfully.
    Connected,
    /// The last sync session with this peer ended with an error.
    SyncFailed,
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
            StreamEvent::SyncEnded {
                remote_node_id,
                error,
                ..
            } => {
                let new_status = if error.is_some() {
                    ConnectionStatus::SyncFailed
                } else {
                    ConnectionStatus::Connected
                };
                self.connections.insert(*remote_node_id, new_status);
            }
            _ => {}
        }
    }

    pub fn connections(&self) -> &HashMap<VerifyingKey, ConnectionStatus> {
        &self.connections
    }
}
