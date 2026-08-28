use std::collections::HashSet;
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use p2panda::Node;
use p2panda::NodeId;
use p2panda::network::NetworkError;
use p2panda::node::SpawnError;
use p2panda::streams::{
    EphemeralPublishError, EphemeralStreamPublisher, EphemeralStreamSubscription, PublishError, StreamEvent, StreamFrom, StreamPublisher,
    StreamSubscription,
};
use p2panda_core::{Hash, SigningKey, Topic, VerifyingKey};
use p2panda_net::iroh_endpoint::RelayUrl;
use p2panda_store::SqliteError;
use sqlx::Row;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use thiserror::Error;
use tokio::sync::{RwLock, mpsc};
use tokio::time::interval;
use tokio_stream::StreamExt;

use crate::RegionAdminTopic;
use crate::node_status::NodeStatus;
use crate::region::{RegionId, RegionTopic};

static DEFAULT_IROH_RELAY_URL: LazyLock<RelayUrl> =
    LazyLock::new(|| "https://euc1-1.relay.n0.iroh-canary.iroh.link".parse().expect("valid relay URL"));

#[derive(Clone)]
pub struct IncomingOperation {
    pub author: VerifyingKey,
    pub topic: Topic,
    pub bytes: Vec<u8>,
    pub operation_id: Hash,
    pub received_timestamp: u64,
}

#[derive(Debug, Error)]
pub enum PandaNodeError {
    #[error(transparent)]
    NodeSpawn(#[from] SpawnError),
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

#[derive(Debug, Error)]
pub enum PandaPublishError {
    #[error("Node not started")]
    NodeNotStarted,
    #[error("No subscription found for topic {0:?}")]
    NoSubscription(Topic),
    #[error(transparent)]
    Publish(#[from] PublishError),
    #[error(transparent)]
    EphemeralPublish(#[from] EphemeralPublishError),
    #[error("App error: {0}")]
    AppError(String),
}

#[derive(Debug, Error)]
pub enum SubscriptionError {
    #[error("Already subscribed to topic {0:?}")]
    AlreadySubscribed(Topic),
    #[error(transparent)]
    CreateStream(#[from] p2panda::node::CreateStreamError),
}

pub struct RequiredNodeParams {
    pub private_key: SigningKey,
    pub network_id: Hash,
    pub bootstrap_node_ids: Vec<VerifyingKey>,
    pub relay_url: Option<RelayUrl>,
}

struct Publisher {
    topic: Topic,
    stream_publisher: StreamPublisher<Vec<u8>>,
    ephemeral_publisher: EphemeralStreamPublisher<Vec<u8>>,
}

pub struct PandaNode {
    network: RwLock<Node>,
    publishers: RwLock<Vec<Publisher>>,
    regions: RwLock<HashSet<RegionId>>,
    node_status: Arc<RwLock<NodeStatus>>,
    pool: SqlitePool,
    pub public_key: VerifyingKey,
}

impl PandaNode {
    pub async fn new(params: &RequiredNodeParams, database_url: &str) -> Result<Self, PandaNodeError> {
        let public_key = params.private_key.verifying_key();
        let network_id: [u8; 32] = *params.network_id.as_bytes();

        let mut builder = Node::builder()
            .network_id(network_id)
            .signing_key(params.private_key.clone())
            .database_url(database_url);

        if cfg!(not(test)) {
            let best_relay_url = params.relay_url.clone().unwrap_or_else(|| DEFAULT_IROH_RELAY_URL.clone());

            for bootstrap_id in &params.bootstrap_node_ids {
                builder = builder.bootstrap(*bootstrap_id, best_relay_url.clone());
            }
            builder = builder.relay_url(best_relay_url);
        }

        let node = builder.spawn().await?;

        // Open a read-only pool against the same file for diagnostic queries.
        let pool = open_pool(database_url).await?;

        Ok(Self {
            network: RwLock::new(node),
            publishers: RwLock::new(Vec::new()),
            regions: RwLock::new(HashSet::new()),
            node_status: Arc::new(RwLock::new(NodeStatus::new())),
            pool,
            public_key,
        })
    }

    async fn subscribe_to_stream(
        &self,
        topic_id: &Topic,
        events_tx: mpsc::Sender<IncomingOperation>,
        mut subscription: StreamSubscription<Vec<u8>>,
    ) {
        let topic_status = self.node_status.write().await.register_topic(*topic_id);

        tokio::spawn(async move {
            while let Some(event) = subscription.next().await {
                match event {
                    StreamEvent::Processed { operation: op, .. } => {
                        let incoming = IncomingOperation {
                            author: op.author(),
                            topic: op.topic(),
                            bytes: op.message().clone(),
                            operation_id: op.id(),
                            received_timestamp: op.timestamp(),
                        };
                        if events_tx.send(incoming).await.is_err() {
                            break;
                        }
                    }
                    StreamEvent::DecodeFailed { error, .. } => {
                        tracing::error!("failed decoding incoming operation: {error}");
                    }
                    StreamEvent::ReplayFailed { error, .. } => {
                        tracing::error!("error replaying operation stream: {error}");
                    }
                    event @ (StreamEvent::SyncStarted { .. } | StreamEvent::SyncEnded { .. }) => {
                        topic_status.write().await.handle_stream_event(&event);
                    }
                    StreamEvent::ImportStarted { .. } | StreamEvent::ImportEnded { .. } => {}
                    StreamEvent::ReplayStarted { .. } | StreamEvent::ReplayEnded => {}
                    StreamEvent::ProcessingFailed { error, .. } => {
                        tracing::error!("operation processing failed: {error}");
                    }
                    StreamEvent::AckFailed { error, .. } => {
                        tracing::error!("operation ack failed: {error}");
                    }
                }
            }
        });
    }

    async fn subscribe_to_ephemeral_stream(
        &self,
        events_tx: mpsc::Sender<IncomingOperation>,
        mut subscription: EphemeralStreamSubscription<Vec<u8>>,
    ) {
        tokio::spawn(async move {
            while let Some(event) = subscription.next().await {
                let incoming = IncomingOperation {
                    author: event.author(),
                    topic: event.topic(),
                    bytes: event.body().clone(),
                    received_timestamp: event.timestamp(),
                    operation_id: Hash::digest(event.body().clone()),
                };
                if events_tx.send(incoming).await.is_err() {
                    break;
                }
            }
        });
    }

    async fn subscribe_to_topic(&self, topic_id: Topic, events_tx: mpsc::Sender<IncomingOperation>) -> Result<(), SubscriptionError> {
        if self.publishers.read().await.iter().any(|p| &p.topic == &topic_id) {
            return Err(SubscriptionError::AlreadySubscribed(topic_id));
        }

        let network = self.network.read().await;
        let (stream_publisher, stream_subscription) = network.stream_from::<Vec<u8>>(topic_id, StreamFrom::Frontier).await?;

        let (ephemeral_publisher, ephemeral_subscription) = network.ephemeral_stream::<Vec<u8>>(topic_id).await?;

        drop(network);

        let publisher = Publisher {
            topic: topic_id,
            stream_publisher,
            ephemeral_publisher,
        };

        self.publishers.write().await.push(publisher);
        self.subscribe_to_stream(&topic_id, events_tx.clone(), stream_subscription).await;
        self.subscribe_to_ephemeral_stream(events_tx.clone(), ephemeral_subscription).await;

        Ok(())
    }

    pub async fn get_subscribed_topics(&self) -> Vec<Topic> {
        self.publishers.read().await.iter().map(|p| &p.topic).cloned().collect()
    }

    /// Returns the shared [`NodeStatus`] covering all subscribed topics.
    pub async fn get_node_status(&self) -> Arc<RwLock<NodeStatus>> {
        self.node_status.clone()
    }

    /// Record that this node is participating in `region_id`. Used by
    /// [`Self::get_regions`] and exposed via the gRPC `ListRegions` RPC.
    pub async fn register_region(&self, region_id: RegionId) {
        self.regions.write().await.insert(region_id);
    }

    /// Returns all registered region IDs.
    pub async fn get_regions(&self) -> Vec<RegionId> {
        self.regions.read().await.iter().cloned().collect()
    }

    /// Insert a bootstrap node at runtime.
    pub async fn insert_bootstrap(&self, node_id: NodeId, relay_url: Option<RelayUrl>) -> Result<(), NetworkError> {
        let relay_url = relay_url.unwrap_or_else(|| DEFAULT_IROH_RELAY_URL.clone());
        let network = self.network.read().await;

        network.insert_bootstrap(node_id, relay_url).await
    }

    /// Creates a new `StreamFrom::Start` stream for `topic_id` and forwards every
    /// `StreamEvent::Processed` operation to `events_tx`.  Unlike
    /// `subscribe_to_topic` this does not guard against the topic already being
    /// subscribed, so it can be called while a live frontier subscription is
    /// active.  The publisher half of the stream is dropped immediately because
    /// publishing is handled by the existing subscription.
    pub async fn replay_topic(&self, topic_id: Topic, events_tx: mpsc::Sender<IncomingOperation>) -> Result<(), SubscriptionError> {
        let network = self.network.read().await;
        let (_publisher, mut subscription) = network.stream_from::<Vec<u8>>(topic_id, StreamFrom::Start).await?;
        drop(network);

        tokio::spawn(async move {
            while let Some(event) = subscription.next().await {
                match event {
                    StreamEvent::Processed { operation: op, .. } => {
                        let incoming = IncomingOperation {
                            author: op.author(),
                            topic: op.topic(),
                            bytes: op.message().clone(),
                            operation_id: op.id(),
                            received_timestamp: op.timestamp(),
                        };
                        if events_tx.send(incoming).await.is_err() {
                            break;
                        }
                    }
                    StreamEvent::DecodeFailed { error, .. } => {
                        tracing::error!("failed decoding operation during replay: {error}");
                    }
                    StreamEvent::ReplayFailed { error, .. } => {
                        tracing::error!("error during operation replay: {error}");
                    }
                    StreamEvent::SyncStarted { .. } | StreamEvent::SyncEnded { .. } => {}
                    StreamEvent::ImportStarted { .. } | StreamEvent::ImportEnded { .. } => {}
                    StreamEvent::ReplayStarted { .. } | StreamEvent::ReplayEnded => {}
                    StreamEvent::ProcessingFailed { error, .. } => {
                        tracing::error!("operation processing failed during replay: {error}");
                    }
                    StreamEvent::AckFailed { error, .. } => {
                        tracing::error!("operation ack failed during replay: {error}");
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn subscribe_to_region_topic<T: RegionTopic>(
        &self,
        region_topic: &T,
        events_tx: mpsc::Sender<IncomingOperation>,
    ) -> Result<(), SubscriptionError> {
        let topic = region_topic.p2panda_topic();
        self.subscribe_to_topic(topic, events_tx).await
    }

    pub async fn publish_to_region_topic<T: RegionTopic>(&self, region_topic: &T, bytes: Vec<u8>) -> Result<Hash, PandaPublishError> {
        let topic = region_topic.p2panda_topic();
        self.publish(topic, bytes).await
    }

    async fn publish(&self, topic_id: Topic, bytes: Vec<u8>) -> Result<Hash, PandaPublishError> {
        let publishers = self.publishers.read().await;
        let publisher = publishers
            .iter()
            .find(|p| p.topic == topic_id)
            .ok_or(PandaPublishError::NoSubscription(topic_id))?;
        let publish_future = publisher.stream_publisher.publish(bytes).await?;
        Ok(publish_future.hash())
    }

    pub async fn publish_ephemeral_to_region_topic<T: RegionTopic>(
        &self,
        region_topic: &T,
        bytes: Vec<u8>,
    ) -> Result<(), PandaPublishError> {
        // to do - where should the publishing happen?
        let topic_id = region_topic.p2panda_topic();
        let publishers = self.publishers.read().await;
        let publisher = publishers
            .iter()
            .find(|p| p.topic == topic_id)
            .ok_or(PandaPublishError::NoSubscription(topic_id))?;
        let regions = self.get_regions().await;

        for region in regions {
            publisher.ephemeral_publisher.publish(bytes.clone()).await?;
        }

        Ok(())
    }

    pub async fn get_log_counts(&self) -> Result<Vec<LogCount>, SqliteError> {
        let rows = sqlx::query("SELECT public_key, COUNT(*) AS total FROM operations_v1 GROUP BY public_key")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| LogCount {
                node_id: row.get("public_key"),
                total: row.get("total"),
            })
            .collect())
    }

    pub async fn get_operation_counts_by_topic(&self) -> Result<Vec<OperationCountByAuthorAndTopic>, SqliteError> {
        let rows = sqlx::query(
            "SELECT lower(hex(substr(t.topic, 3))) AS topic_hex, t.author, COUNT(o.hash) AS total
             FROM topics_v1 t
             JOIN operations_v1 o ON o.verifying_key = t.author AND o.log_id = t.data_id
             GROUP BY t.topic, t.author",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| OperationCountByAuthorAndTopic {
                topic_hex: row.get("topic_hex"),
                author_node_id: row.get("author"),
                count: row.get("total"),
            })
            .collect())
    }
}

pub struct LogCount {
    pub node_id: String,
    pub total: i64,
}

pub struct OperationCountByAuthorAndTopic {
    pub topic_hex: String,
    pub author_node_id: String,
    pub count: i64,
}

async fn open_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // Strip the "sqlite:" scheme prefix if present since SqliteConnectOptions
    // wants just the path.
    let path = database_url.strip_prefix("sqlite:").unwrap_or(database_url);

    let options = SqliteConnectOptions::new().filename(path).create_if_missing(true);

    SqlitePoolOptions::new().max_connections(4).connect_with(options).await
}
