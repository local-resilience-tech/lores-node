use futures::StreamExt;
use tokio::sync::broadcast;

use crate::node::AppNodeOperation;
use crate::store::{OperationStream, RawOperationEvent, StoreError};

/// Deserializes raw operation payloads from a stream and broadcasts them to
/// all subscribers of the event channel.
///
/// Generic over the inner user `Op`; always broadcasts `AppNodeOperation<Op>`.
pub(crate) struct OperationConsumer<Op> {
    event_tx: broadcast::Sender<AppNodeOperation<Op>>,
}

impl<Op> Clone for OperationConsumer<Op> {
    fn clone(&self) -> Self {
        Self {
            event_tx: self.event_tx.clone(),
        }
    }
}

impl<Op: Clone + Send + 'static> OperationConsumer<Op> {
    pub(crate) fn new(event_tx: broadcast::Sender<AppNodeOperation<Op>>) -> Self {
        Self { event_tx }
    }

    pub(crate) fn subscribe(&self) -> broadcast::Receiver<AppNodeOperation<Op>> {
        self.event_tx.subscribe()
    }

    pub(crate) fn send(&self, op: AppNodeOperation<Op>) {
        let _ = self.event_tx.send(op);
    }

    /// Drain a stream of raw events, deserializing and broadcasting each one.
    ///
    /// Returns `Ok(count)` if the stream ended cleanly, or `Err` on the first
    /// stream-level failure. Deserialization failures are logged as warnings
    /// and do not stop the drain.
    pub(crate) async fn drain_stream(&self, stream: &mut OperationStream) -> Result<usize, StoreError>
    where
        Op: for<'de> serde::Deserialize<'de>,
    {
        let mut count = 0usize;
        while let Some(item) = stream.next().await {
            match item {
                Ok(RawOperationEvent { payload, author, operation_id, timestamp }) => {
                    match serde_json::from_slice::<Op>(&payload) {
                        Ok(op) => {
                            let _ = self.event_tx.send(AppNodeOperation {
                                op,
                                author,
                                operation_id,
                                timestamp,
                            });
                            count += 1;
                        }
                        Err(e) => tracing::warn!("Failed to deserialize operation: {e}"),
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(count)
    }
}
