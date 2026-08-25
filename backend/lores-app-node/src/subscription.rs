use std::sync::Arc;

use crate::backoff::Backoff;
use crate::consumer::OperationConsumer;
use crate::node::{map_store_error, NodeError};
use crate::store::{OperationStore, StoreError};
use tokio::sync::{watch, Mutex};

/// Drives a remote subscription in a loop, reconnecting with exponential
/// backoff on any failure.
pub(crate) struct LiveSubscription<Op> {
    operation_store: Arc<Mutex<Box<dyn OperationStore>>>,
    consumer: OperationConsumer<Op>,
    error_tx: watch::Sender<Option<NodeError>>,
}

impl<Op: Clone + Send + 'static> LiveSubscription<Op> {
    pub(crate) fn new(
        operation_store: Arc<Mutex<Box<dyn OperationStore>>>,
        consumer: OperationConsumer<Op>,
        error_tx: watch::Sender<Option<NodeError>>,
    ) -> Self {
        Self {
            operation_store,
            consumer,
            error_tx,
        }
    }

    /// Run the subscription loop forever. Call with `tokio::spawn`.
    pub(crate) async fn run(&self)
    where
        Op: for<'de> serde::Deserialize<'de>,
    {
        let mut backoff = Backoff::new();

        loop {
            let Some(mut stream) = self.try_subscribe(&mut backoff).await else {
                continue;
            };

            if let Err(e) = self.consumer.drain_stream(&mut stream).await {
                self.handle_mid_stream_error(e);
                backoff.reset();
            }

            tracing::info!("Subscription stream ended, reconnecting…");
        }
    }

    async fn try_subscribe(&self, backoff: &mut Backoff) -> Option<crate::store::OperationStream> {
        match self.operation_store.lock().await.subscribe().await {
            Ok(s) => {
                self.error_tx.send_replace(None);
                backoff.reset();
                Some(s)
            }
            Err(err @ StoreError::RegionNotBound(_)) => {
                tracing::warn!(
                    "Subscribe failed — region not bound (retrying in {:?})",
                    backoff.current
                );
                backoff
                    .set_error_and_advance(&self.error_tx, map_store_error(err))
                    .await;
                None
            }
            Err(err @ StoreError::Other(_)) => {
                tracing::error!("Subscribe failed: {err} (retrying in {:?})", backoff.current);
                backoff
                    .set_error_and_advance(&self.error_tx, map_store_error(err))
                    .await;
                None
            }
        }
    }

    fn handle_mid_stream_error(&self, err: StoreError) {
        tracing::warn!("Stream disconnected (reconnecting): {err}");
        self.error_tx.send_replace(Some(map_store_error(err)));
    }
}
