use std::sync::Arc;

use lores_p2panda_client::{PandaClient, SubscriptionFrom};

use crate::backoff::Backoff;
use crate::consumer::OperationConsumer;
use crate::node::{NodeError, map_transport_error};
use crate::transports::{OperationTransport, TransportError};
use crate::types::NodeEvent;
use tokio::sync::{Mutex, broadcast, watch};

/// Drives a remote subscription in a loop, reconnecting with exponential
/// backoff on any failure.
pub(crate) struct LiveSubscription<Op> {
    transport: Arc<Mutex<Box<dyn OperationTransport>>>,
    consumer: OperationConsumer<Op>,
    error_tx: watch::Sender<Option<NodeError>>,
    node_event_tx: broadcast::Sender<NodeEvent>,
    panda_client: Option<Arc<Mutex<PandaClient>>>,
    app_id: String,
    instance_id: String,
    start_from: SubscriptionFrom,
}

impl<Op: Clone + Send + 'static> LiveSubscription<Op> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        transport: Arc<Mutex<Box<dyn OperationTransport>>>,
        consumer: OperationConsumer<Op>,
        error_tx: watch::Sender<Option<NodeError>>,
        node_event_tx: broadcast::Sender<NodeEvent>,
        panda_client: Option<Arc<Mutex<PandaClient>>>,
        app_id: String,
        instance_id: String,
        start_from: SubscriptionFrom,
    ) -> Self {
        Self {
            transport,
            consumer,
            error_tx,
            node_event_tx,
            panda_client,
            app_id,
            instance_id,
            start_from,
        }
    }

    /// Run the subscription loop forever. Call with `tokio::spawn`.
    pub(crate) async fn run(&self)
    where
        Op: for<'de> serde::Deserialize<'de>,
    {
        let mut backoff = Backoff::new();

        loop {
            let Some(mut stream) = self.try_subscribe(self.start_from, &mut backoff).await else {
                continue;
            };

            if let Err(e) = self.consumer.drain_stream(&mut stream, &self.node_event_tx).await {
                self.handle_mid_stream_error(e);
                backoff.reset();
            }

            let _ = self.node_event_tx.send(NodeEvent::ServerDisconnected);
            tracing::info!("Subscription stream ended, reconnecting…");
        }
    }

    async fn try_subscribe(&self, start_from: SubscriptionFrom, backoff: &mut Backoff) -> Option<crate::transports::OperationStream> {
        match self.transport.lock().await.subscribe(start_from).await {
            Ok(s) => {
                self.error_tx.send_replace(None);
                backoff.reset();
                self.fetch_and_emit_server_info().await;
                Some(s)
            }
            Err(err @ TransportError::RegionNotBound(_)) => {
                tracing::warn!("Subscribe failed — region not bound (retrying in {:?})", backoff.current);
                backoff.set_error_and_advance(&self.error_tx, map_transport_error(err)).await;
                None
            }
            Err(err @ TransportError::Other(_)) => {
                tracing::error!("Subscribe failed: {err} (retrying in {:?})", backoff.current);
                backoff.set_error_and_advance(&self.error_tx, map_transport_error(err)).await;
                None
            }
        }
    }

    async fn fetch_and_emit_server_info(&self) {
        let Some(client) = &self.panda_client else {
            return;
        };
        match client.lock().await.info(&self.app_id, &self.instance_id).await {
            Ok(info) => {
                let _ = self.node_event_tx.send(NodeEvent::ServerConnected {
                    node_id: info.node_id,
                    region: crate::types::RegionInfo {
                        region_id: info.region.region_id,
                        slug: info.region.slug,
                        name: info.region.name,
                    },
                });
            }
            Err(e) => tracing::warn!("Failed to fetch server info: {e}"),
        }
    }

    fn handle_mid_stream_error(&self, err: TransportError) {
        tracing::warn!("Stream disconnected (reconnecting): {err}");
        self.error_tx.send_replace(Some(map_transport_error(err)));
    }
}
