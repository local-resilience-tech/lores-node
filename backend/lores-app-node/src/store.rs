use std::future::Future;
use std::pin::Pin;

use futures::Stream;

/// Error returned by [`OperationStore`] methods.
#[derive(Debug)]
pub enum StoreError {
    /// No region has been bound to the given app/instance on the server.
    RegionNotBound(String),
    /// Any other error.
    Other(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::RegionNotBound(msg) => write!(f, "{msg}"),
            StoreError::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for StoreError {}

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
        Self { payload, author: None, operation_id: None, timestamp: None }
    }
}

/// A boxed, heap-allocated stream of raw operation events.
pub(crate) type OperationStream = Pin<Box<dyn Stream<Item = Result<RawOperationEvent, StoreError>> + Send>>;

/// Internal trait over raw-bytes operation delivery.
///
/// App developers never interact with this directly — they use [`crate::AppNode`]
/// and its named constructors (`grpc`, etc.).
pub(crate) trait OperationStore: Send + Sync + 'static {
    fn publish(
        &mut self,
        payload: Vec<u8>,
        idempotency_key: Option<String>,
    ) -> Pin<Box<dyn Future<Output = Result<(), StoreError>> + Send + '_>>;

    /// Open a subscription to incoming operations.
    ///
    /// The outer `Result` covers connection-time errors (e.g. `RegionNotBound`).
    /// The inner stream yields individual operation payloads or per-item errors.
    fn subscribe(&mut self) -> Pin<Box<dyn Future<Output = Result<OperationStream, StoreError>> + Send + '_>>;

    /// Replay all operations in insertion order.
    fn replay(&mut self) -> Pin<Box<dyn Future<Output = Result<OperationStream, StoreError>> + Send + '_>> {
        Box::pin(async move {
            let s: OperationStream = Box::pin(futures::stream::empty());
            Ok(s)
        })
    }
}
