use std::sync::Arc;

use lores_p2panda_client::PandaClient;

use crate::backoff::Backoff;
use crate::consumer::OperationConsumer;
use crate::node::{map_store_error, NodeError};
use crate::stores::{OperationStore, StoreError};
use crate::types::NodeEvent;
use tokio::sync::{broadcast, watch, Mutex};

/// Drives a remote subscription in a loop, reconnecting with exponential
/// backoff on any failure.
pub(crate) struct LiveSubscription<Op> {
    operation_store: Arc<Mutex<Box<dyn OperationStore>>>,
    consumer: OperationConsumer<Op>,
    error_tx: watch::Sender<Option<NodeError>>,
    node_event_tx: broadcast::Sender<NodeEvent>,
    panda_client: Option<Arc<Mutex<PandaClient>>>,
    instance_id: String,
}

impl<Op: Clone + Send + 'static> LiveSubscription<Op> {
    pub(crate) fn new(
        operation_store: Arc<Mutex<Box<dyn OperationStore>>>,
        consumer: OperationConsumer<Op>,
        error_tx: watch::Sender<Option<NodeError>>,
        node_event_tx: broadcast::Sender<NodeEvent>,
        panda_client: Option<Arc<Mutex<PandaClient>>>,
        instance_id: String,
    ) -> Self {
        Self {
            operation_store,
            consumer,
            error_tx,
            node_event_tx,
            panda_client,
            instance_id,
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

            let _ = self.node_event_tx.send(NodeEvent::ServerDisconnected);
            tracing::info!("Subscription stream ended, reconnecting…");
        }
    }

    async fn try_subscribe(&self, backoff: &mut Backoff) -> Option<crate::stores::OperationStream> {
        match self.operation_store.lock().await.subscribe().await {
            Ok(s) => {
                self.error_tx.send_replace(None);
                backoff.reset();
                self.fetch_and_emit_server_info().await;
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
                tracing::error!(
                    "Subscribe failed: {err} (retrying in {:?})",
                    backoff.current
                );
                backoff
                    .set_error_and_advance(&self.error_tx, map_store_error(err))
                    .await;
                None
            }
        }
    }

    async fn fetch_and_emit_server_info(&self) {
        let Some(client) = &self.panda_client else {
            return;
        };
        match client.lock().await.info(&self.instance_id).await {
            Ok(node_id) => {
                let _ = self.node_event_tx.send(NodeEvent::ServerConnected {
                    node_id: crate::types::LoResNodeId(node_id.0),
                });
            }
            Err(e) => tracing::warn!("Failed to fetch server info: {e}"),
        }
    }

    fn handle_mid_stream_error(&self, err: StoreError) {
        tracing::warn!("Stream disconnected (reconnecting): {err}");
        self.error_tx.send_replace(Some(map_store_error(err)));
    }
}
