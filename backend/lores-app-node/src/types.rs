use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// 32-byte p2panda public key identifying a remote author.
#[derive(Clone, Serialize, Deserialize)]
pub struct LoResNodeId(pub Vec<u8>);

impl LoResNodeId {
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }
}

impl fmt::Debug for LoResNodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LoResNodeId({})", self.to_hex())
    }
}

/// 32-byte p2panda operation hash.
#[derive(Clone, Serialize, Deserialize)]
pub struct LoResOperationId(pub Vec<u8>);

impl LoResOperationId {
    pub fn to_hex(&self) -> String {
        hex::encode(&self.0)
    }
}

impl fmt::Debug for LoResOperationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LoResOperationId({})", self.to_hex())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AppNodeOperation<Op> {
    pub op: Op,
    /// Stable local identity assigned at publish time, before any network round-trip.
    pub local_operation_id: Option<Uuid>,
    /// 32-byte p2panda operation hash. `None` for locally-published operations.
    pub panda_operation_id: Option<LoResOperationId>,
    /// p2panda public key of the node that authored the operation.
    pub author_node_id: Option<LoResNodeId>,
    pub timestamp: Option<u64>,
}

#[derive(Clone, Debug)]
pub enum NodeEvent {
    ServerConnected { node_id: LoResNodeId },
    ServerDisconnected,
}
