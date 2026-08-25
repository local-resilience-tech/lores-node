use serde::{Deserialize, Serialize};

/// 32-byte p2panda public key identifying a remote author.
#[derive(Clone, Serialize, Deserialize)]
pub struct LoResNodeId(pub Vec<u8>);

/// 32-byte p2panda operation hash.
#[derive(Clone, Serialize, Deserialize)]
pub struct LoResOperationId(pub Vec<u8>);

#[derive(Clone, Serialize, Deserialize)]
pub struct AppNodeOperation<Op> {
    pub op: Op,
    /// `None` for locally-published operations.
    pub node: Option<LoResNodeId>,
    /// 32-byte p2panda operation hash. `None` for locally-published operations.
    pub operation_id: Option<LoResOperationId>,
    /// Unix timestamp in milliseconds. `None` for locally-published operations.
    pub timestamp: Option<u64>,
}
