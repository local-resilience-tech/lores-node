use std::future::Future;
use std::pin::Pin;

use futures::Stream;
use lores_p2panda_client::SubscriptionFrom;

use crate::types::{NodeId, OperationId};

/// Result returned by [`OperationTransport::publish`].
pub(crate) struct TransportPublishResult {
    /// p2panda operation hash, if the backend can provide it synchronously.
    pub operation_id: Option<OperationId>,
    /// Identity of the node that persisted the operation, if known.
    pub node_id: Option<NodeId>,
}

/// Error returned when publishing, subscribing to, or replaying operations.
#[derive(Debug)]
pub enum TransportError {
    /// No region has been bound to the given app/instance on the server.
    RegionNotBound(String),
    /// Any other error.
    Other(String),
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportError::RegionNotBound(msg) => write!(f, "{msg}"),
            TransportError::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for TransportError {}

/// Metadata forwarded from the p2panda layer alongside a raw payload.
/// Fields are `None` for locally-originated operations (pre-network assignment).
pub(crate) struct RawOperationEvent {
    pub payload: Vec<u8>,
    /// 32-byte p2panda author public key.
    pub author: Option<Vec<u8>>,
    /// 32-byte p2panda operation hash.
    pub operation_id: Option<Vec<u8>>,
    /// Unix timestamp in milliseconds.
    pub timestamp: Option<u64>,
}

impl RawOperationEvent {
    /// Construct an event for a locally-published operation with no p2panda metadata.
    pub(crate) fn new_local(payload: Vec<u8>) -> Self {
        Self {
            payload,
            author: None,
            operation_id: None,
            timestamp: None,
        }
    }
}

/// An item yielded by an [`OperationTransport::subscribe`] stream: either an
/// operation, or one of p2panda's replay lifecycle events
pub(crate) enum RawEvent {
    Operation(RawOperationEvent),
    #[allow(dead_code)]
    ReplayStarted {
        total_operations: u32,
    },
    ReplayEnded,
}

/// A boxed, heap-allocated stream of raw subscription events.
pub(crate) type OperationStream = Pin<Box<dyn Stream<Item = Result<RawEvent, TransportError>> + Send>>;

/// Internal trait over raw-bytes operation delivery.
///
/// App developers never interact with this directly — they use [`crate::AppNode`]
/// and its named constructors (`grpc`, etc.).
pub(crate) trait OperationTransport: Send + Sync + 'static {
    /// Returns operation metadata if the backend can provide it synchronously.
    fn publish(
        &mut self,
        payload: Vec<u8>,
        idempotency_key: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<TransportPublishResult, TransportError>> + Send + '_>>;

    /// Open a subscription to incoming operations.
    ///
    /// The outer `Result` covers connection-time errors (e.g. `RegionNotBound`).
    /// The inner stream yields individual operation payloads or per-item errors.
    fn subscribe(
        &mut self,
        start_from: SubscriptionFrom,
    ) -> Pin<Box<dyn Future<Output = Result<OperationStream, TransportError>> + Send + '_>>;
}

pub(crate) mod grpc;
pub(crate) mod local;
pub(crate) mod outbox;
