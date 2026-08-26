use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

pub use lores_p2panda_client::{NodeId, OperationId, RegionId};

#[derive(Clone, Debug)]
pub struct RegionInfo {
    pub region_id: RegionId,
}

impl fmt::Display for RegionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.region_id, f)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AppNodeOperation<Op> {
    pub op: Op,
    /// Stable local identity assigned at publish time, before any network round-trip.
    pub local_operation_id: Option<Uuid>,
    /// 32-byte p2panda operation hash. `None` for locally-published operations.
    pub panda_operation_id: Option<OperationId>,
    /// p2panda public key of the node that authored the operation.
    pub author_node_id: Option<NodeId>,
    pub timestamp: Option<u64>,
}

#[derive(Clone, Debug)]
pub enum NodeEvent {
    ServerConnected { node_id: NodeId, region: RegionInfo },
    ServerDisconnected,
}
