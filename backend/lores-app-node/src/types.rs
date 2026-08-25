use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// 32-byte p2panda public key identifying a remote author.
#[derive(Clone, Serialize, Deserialize)]
pub struct LoResNodeId(pub Vec<u8>);

impl fmt::Debug for LoResNodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LoResNodeId({})", hex::encode(&self.0))
    }
}

/// 32-byte p2panda operation hash.
#[derive(Clone, Serialize, Deserialize)]
pub struct LoResOperationId(pub Vec<u8>);

impl fmt::Debug for LoResOperationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LoResOperationId({})", hex::encode(&self.0))
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AppNodeOperation<Op> {
    pub op: Op,
    /// Stable local identity assigned at publish time, before any network round-trip.
    pub local_operation_id: Option<Uuid>,
    /// `None` for locally-published operations.
    pub panda_operation_id: Option<LoResOperationId>,
    /// Unix timestamp in milliseconds. `None` for locally-published operations.
    pub node: Option<LoResNodeId>,
    /// 32-byte p2panda operation hash. `None` for locally-published operations.
    pub timestamp: Option<u64>,
}
