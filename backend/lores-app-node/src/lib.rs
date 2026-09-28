#![doc = include_str!("../README.md")]

mod backoff;
mod consumer;
mod node;
mod projection;
mod subscription;
mod transports;
mod types;

pub use lores_p2panda_client::{GetNodeError, NodeInfo};
pub use node::{AppNode, ConnectError, NodeError};
pub use projection::ProjectionDb;
pub use transports::TransportError;
pub use types::{AppNodeOperation, NodeEvent, NodeId, OperationId, RegionId, RegionInfo};
