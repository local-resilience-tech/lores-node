use std::borrow::Borrow;
use std::collections::HashSet;
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use p2panda::Node;
use p2panda::NodeId;
use p2panda::network::NetworkError;
use p2panda::node::SpawnError;
use p2panda::operation::{Header, LogId};
use p2panda::streams::{
    EphemeralPublishError, EphemeralStreamPublisher, EphemeralStreamSubscription, PublishError, StreamEvent, StreamFrom, StreamPublisher,
    StreamSubscription,
};
use p2panda_core::{Hash, SeqNum, SigningKey, Topic, VerifyingKey};
use p2panda_encryption::Rng;
use p2panda_encryption::crypto::x25519::SecretKey;
use p2panda_net::iroh_endpoint::RelayUrl;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use thiserror::Error;
use tokio::sync::{RwLock, mpsc};
use tokio::time::interval;
use tokio_stream::StreamExt;

use crate::RegionAdminTopic;
use crate::node_status::NodeStatus;
use crate::region::{RegionId, RegionTopic};
use crate::topic_status::TopicStatus;

static DEFAULT_IROH_RELAY_URL: LazyLock<RelayUrl> =
    LazyLock::new(|| "https://euc1-1.relay.n0.iroh-canary.iroh.link".parse().expect("valid relay URL"));

/// Internal mirror of `p2panda::credentials::Inner` so we can build a
/// `p2panda::Credentials` from an existing signing key and identity secret.
#[derive(Serialize, Deserialize)]
struct CredentialsInner {
    signing_key: SigningKey,
    identity_secret_key: SecretKey,
}

pub fn credentials_from_seed(signing_key: SigningKey, identity_secret_seed: [u8; 32]) -> p2panda::Credentials {
    let identity_secret_key = SecretKey::from_rng(&Rng::from_seed(identity_secret_seed)).expect("derive identity secret from seed");
    credentials_from_keys(signing_key, identity_secret_key)
}

fn credentials_from_keys(signing_key: SigningKey, identity_secret_key: SecretKey) -> p2panda::Credentials {
    let inner = CredentialsInner {
        signing_key,
        identity_secret_key,
    };
    let value = serde_json::to_value(&inner).expect("serialize credentials inner");
    serde_json::from_value(value).expect("deserialize p2panda credentials")
}

#[derive(Clone)]
pub struct IncomingOperation {
    pub author: VerifyingKey,
    pub topic: Topic,
    pub bytes: Vec<u8>,
    pub operation_id: Hash,
    pub received_timestamp: u64,
    pub log_id: Option<LogId>,
    pub seq_num: Option<SeqNum>,
}

/// Reads the log position of a processed (persisted) operation.
fn log_position<M>(op: &p2panda::streams::ProcessedOperation<M>) -> (Option<LogId>, Option<SeqNum>) {
    let header: &Header = Borrow::<Header>::borrow(&op);
    (Some(header.extensions.log_id()), Some(header.seq_num))
}

/// Events emitted by a named-cursor topic subscription, surfacing p2panda's own
/// replay lifecycle so callers can tell historical operations from live ones.
#[derive(Clone)]
pub enum SubscriptionEvent {
    Operation(Box<IncomingOperation>),
    ReplayStarted { total_operations: u32 },
    ReplayEnded,
}

#[derive(Debug, Error)]
pub enum PandaNodeError {
    #[error(transparent)]
    NodeSpawn(#[from] Box<SpawnError>),
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
    #[error("internal server error")]
    ServerError,
}

pub struct RequiredNodeParams {
    pub credentials: p2panda::Credentials,
    pub network_id: Hash,
    pub bootstrap_node_ids: Vec<VerifyingKey>,
    pub relay_url: Option<RelayUrl>,
}

struct Publisher {
    topic: Topic,
    stream_publisher: Option<StreamPublisher<Vec<u8>>>,
    ephemeral_publisher: Option<EphemeralStreamPublisher<Vec<u8>>>,
}

pub struct PandaNode {
    network: RwLock<Node>,
    publishers: RwLock<Vec<Publisher>>,
    regions: RwLock<HashSet<RegionId>>,
    node_status: Arc<RwLock<NodeStatus>>,
    pool: SqlitePool,
    pub public_key: VerifyingKey,
}

const HEARTBEAT_FREQUENCY_MINS: u64 = 5;

impl PandaNode {
    pub async fn new(params: &RequiredNodeParams, database_url: &str) -> Result<Self, PandaNodeError> {
        let public_key = params.credentials.verifying_key();
        let network_id: [u8; 32] = *params.network_id.as_bytes();

        let mut builder = Node::builder()
            .network_id(network_id)
            .credentials(params.credentials.clone())
            .database_url(database_url);

        if cfg!(not(test)) {
            let best_relay_url = params.relay_url.clone().unwrap_or_else(|| DEFAULT_IROH_RELAY_URL.clone());

            for bootstrap_id in &params.bootstrap_node_ids {
                builder = builder.bootstrap(*bootstrap_id, best_relay_url.clone());
            }
            builder = builder.relay_url(best_relay_url);
        }

        let node = builder.spawn().await.map_err(|err| PandaNodeError::NodeSpawn(Box::new(err)))?;

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
    pub async fn insert_bootstrap(&self, node_id: NodeId, relay_url: Option<RelayUrl>) -> Result<(), Box<NetworkError>> {
        let relay_url = relay_url.unwrap_or_else(|| DEFAULT_IROH_RELAY_URL.clone());
        let network = self.network.read().await;

        network.insert_bootstrap(node_id, relay_url).await.map_err(Box::new)
    }

    pub async fn subscribe_to_region_topic<T: RegionTopic>(
        &self,
        region_topic: &T,
        events_tx: mpsc::Sender<IncomingOperation>,
    ) -> Result<(), SubscriptionError> {
        let topic = region_topic.p2panda_topic();
        self.subscribe_to_topic_persisted(topic, events_tx.clone()).await?;
        self.subscribe_to_topic_ephemeral(topic, events_tx.clone()).await?;

        Ok(())
    }

    /// Replays `region_topic` under its own named cursor, independent of the
    /// node's primary frontier subscription and of any other cursor name on
    /// the same topic. Replays every persisted operation from the beginning
    /// of the log, then emits `ReplayEnded` and stops.
    ///
    /// Reusing the same `cursor_name` for more than one concurrent call is not
    /// recommended.
    pub async fn replay_region_topic_as<T: RegionTopic>(
        &self,
        region_topic: &T,
        cursor_name: impl Into<String>,
        events_tx: mpsc::Sender<SubscriptionEvent>,
    ) -> Result<(), SubscriptionError> {
        let topic_id = region_topic.p2panda_topic();

        let network = self.network.read().await;
        let (stream_publisher, subscription) = network
            .stream_from::<Vec<u8>>(topic_id, StreamFrom::Start, Some(cursor_name.into()))
            .await?;
        drop(network);

        // p2panda's `SyncHandle::drop` tears down the *whole topic's* sync
        // session, not just this handle's — dropping this publisher would
        // silently kill the primary subscription and any other named-cursor
        // subscriptions sharing the topic. Keep it alive indefinitely instead
        // of dropping it, even though this subscription never publishes
        // through it itself.
        self.publishers.write().await.push(Publisher {
            topic: topic_id,
            stream_publisher: Some(stream_publisher),
            ephemeral_publisher: None,
        });

        Self::spawn_stream_task(subscription, events_tx, None, true, Some);

        Ok(())
    }

    pub async fn publish_to_region_topic<T: RegionTopic>(&self, region_topic: &T, bytes: Vec<u8>) -> Result<Hash, PandaPublishError> {
        let topic = region_topic.p2panda_topic();
        self.publish(topic, bytes).await
    }

    pub async fn start_heartbeat_publication(self: Arc<Self>, heartbeat_message_payload: Vec<u8>) -> Result<(), PandaPublishError> {
        tokio::spawn(async move {
            let mut timer = interval(Duration::from_mins(HEARTBEAT_FREQUENCY_MINS));

            loop {
                timer.tick().await;

                for region_id in self.get_regions().await {
                    if let Err(err) = self.publish_single_heartbeat(&heartbeat_message_payload, &region_id).await {
                        eprintln!("Error sending ephemeral message: {}", err);
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn get_log_counts(&self) -> Result<Vec<LogCount>, sqlx::Error> {
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

    pub async fn get_operation_counts_by_topic(&self) -> Result<Vec<OperationCountByAuthorAndTopic>, sqlx::Error> {
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

    pub async fn has_operations_for_topic(&self, topic_id: Topic) -> Result<bool, sqlx::Error> {
        let row = sqlx::query(
            "SELECT EXISTS(
                SELECT 1 FROM topics_v1 t
                JOIN operations_v1 o ON o.verifying_key = t.author AND o.log_id = t.data_id
                WHERE lower(hex(substr(t.topic, 3))) = ?
             ) AS present",
        )
        .bind(topic_id.to_hex())
        .fetch_one(&self.pool)
        .await?;

        Ok(row.get::<i64, _>("present") != 0)
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
                    // Ephemeral messages aren't part of any log.
                    log_id: None,
                    seq_num: None,
                };
                if events_tx.send(incoming).await.is_err() {
                    break;
                }
            }
        });
    }

    async fn subscribe_to_topic_persisted(
        &self,
        topic_id: Topic,
        events_tx: mpsc::Sender<IncomingOperation>,
    ) -> Result<(), SubscriptionError> {
        let mut publishers = self.publishers.write().await;
        let existing_publisher = publishers.iter_mut().find(|p| p.topic == topic_id);

        let network = self.network.read().await;
        let (stream_publisher, stream_subscription) = network.stream_from::<Vec<u8>>(topic_id, StreamFrom::Frontier, None).await?;
        drop(network);

        match existing_publisher {
            Some(p) => {
                if p.stream_publisher.is_some() {
                    return Err(SubscriptionError::AlreadySubscribed(topic_id));
                }

                p.stream_publisher = Some(stream_publisher)
            }
            None => {
                let publisher = Publisher {
                    topic: topic_id,
                    stream_publisher: Some(stream_publisher),
                    ephemeral_publisher: None,
                };

                publishers.push(publisher);
            }
        }

        drop(publishers);

        let topic_status = self.node_status.write().await.register_topic(topic_id);
        Self::spawn_stream_task(stream_subscription, events_tx, Some(topic_status), false, |event| match event {
            SubscriptionEvent::Operation(op) => Some(*op),
            SubscriptionEvent::ReplayStarted { .. } | SubscriptionEvent::ReplayEnded => None,
        });

        Ok(())
    }

    async fn subscribe_to_topic_ephemeral(
        &self,
        topic_id: Topic,
        events_tx: mpsc::Sender<IncomingOperation>,
    ) -> Result<(), SubscriptionError> {
        let mut publishers = self.publishers.write().await;
        let existing_publisher = publishers.iter_mut().find(|p| p.topic == topic_id);

        let network = self.network.read().await;
        let (ephemeral_publisher, ephemeral_subscription) = network.ephemeral_stream::<Vec<u8>>(topic_id).await?;
        drop(network);

        match existing_publisher {
            Some(p) => {
                if p.ephemeral_publisher.is_some() {
                    return Err(SubscriptionError::AlreadySubscribed(topic_id));
                }

                p.ephemeral_publisher = Some(ephemeral_publisher);
            }
            None => {
                let publisher = Publisher {
                    topic: topic_id,
                    stream_publisher: None,
                    ephemeral_publisher: Some(ephemeral_publisher),
                };

                publishers.push(publisher);
            }
        }

        drop(publishers);

        self.subscribe_to_ephemeral_stream(events_tx, ephemeral_subscription).await;

        Ok(())
    }

    /// Spawns a task forwarding every event from `subscription` to `events_tx`,
    /// via `map_event` (returning `None` drops the event). `topic_status`, if
    /// given, is updated on sync events. If `stop_after_replay_ended`, the task
    /// ends right after forwarding `ReplayEnded` instead of continuing to read
    /// from `subscription` (see `replay_region_topic_as`'s doc comment).
    fn spawn_stream_task<O: Send + 'static>(
        mut subscription: StreamSubscription<Vec<u8>>,
        events_tx: mpsc::Sender<O>,
        topic_status: Option<Arc<RwLock<TopicStatus>>>,
        stop_after_replay_ended: bool,
        map_event: impl Fn(SubscriptionEvent) -> Option<O> + Send + 'static,
    ) {
        tokio::spawn(async move {
            while let Some(event) = subscription.next().await {
                match event {
                    StreamEvent::Processed { operation: op, .. } => {
                        let (log_id, seq_num) = log_position(&op);
                        let incoming = IncomingOperation {
                            author: op.author(),
                            topic: op.topic(),
                            bytes: op.message().clone(),
                            operation_id: op.id(),
                            received_timestamp: op.timestamp(),
                            log_id,
                            seq_num,
                        };
                        if let Some(mapped) = map_event(SubscriptionEvent::Operation(Box::new(incoming)))
                            && events_tx.send(mapped).await.is_err()
                        {
                            break;
                        }
                    }
                    StreamEvent::ReplayStarted { total_operations } => {
                        if let Some(mapped) = map_event(SubscriptionEvent::ReplayStarted { total_operations })
                            && events_tx.send(mapped).await.is_err()
                        {
                            break;
                        }
                    }
                    StreamEvent::ReplayEnded => {
                        if let Some(mapped) = map_event(SubscriptionEvent::ReplayEnded) {
                            let _ = events_tx.send(mapped).await;
                        }
                        if stop_after_replay_ended {
                            break;
                        }
                    }
                    StreamEvent::DecodeFailed { error, .. } => {
                        tracing::error!("failed decoding operation: {error}");
                    }
                    StreamEvent::ReplayFailed { error, .. } => {
                        tracing::error!("error replaying operation stream: {error}");
                    }
                    event @ (StreamEvent::SyncStarted { .. } | StreamEvent::SyncEnded { .. }) => {
                        if let Some(status) = &topic_status {
                            status.write().await.handle_stream_event(&event);
                        }
                    }
                    StreamEvent::ImportStarted { .. } | StreamEvent::ImportEnded { .. } => {}
                    StreamEvent::ProcessingFailed { error, .. } => {
                        tracing::error!("operation processing failed: {error}");
                    }
                    StreamEvent::AckFailed { error, .. } => {
                        tracing::error!("operation ack failed: {error}");
                    }
                    StreamEvent::Space { .. } | StreamEvent::Member(_) => {}
                }
            }
        });
    }

    async fn stream_publisher_for(&self, topic_id: Topic) -> Result<StreamPublisher<Vec<u8>>, PandaPublishError> {
        self.publishers
            .read()
            .await
            .iter()
            .find_map(|p| (p.topic == topic_id).then(|| p.stream_publisher.clone()).flatten())
            .ok_or(PandaPublishError::NoSubscription(topic_id))
    }

    async fn ephemeral_publisher_for(&self, topic_id: Topic) -> Result<EphemeralStreamPublisher<Vec<u8>>, PandaPublishError> {
        self.publishers
            .read()
            .await
            .iter()
            .find_map(|p| (p.topic == topic_id).then(|| p.ephemeral_publisher.clone()).flatten())
            .ok_or(PandaPublishError::NoSubscription(topic_id))
    }

    async fn publish(&self, topic_id: Topic, bytes: Vec<u8>) -> Result<Hash, PandaPublishError> {
        let publisher = self.stream_publisher_for(topic_id).await?;
        Ok(publisher.publish(bytes).await?.hash())
    }

    async fn publish_single_heartbeat(&self, heartbeat_message_payload: &[u8], region_id: &RegionId) -> Result<(), PandaPublishError> {
        let topic_id = RegionAdminTopic::new(region_id.clone()).p2panda_topic();
        let publisher = self.ephemeral_publisher_for(topic_id).await?;
        publisher.publish(heartbeat_message_payload.to_vec()).await?;

        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::region::RegionAppTopic;

    #[test]
    fn credentials_from_seed_roundtrip() {
        let signing_key = SigningKey::from_bytes(&[1; 32]);
        let creds = credentials_from_seed(signing_key.clone(), [2; 32]);
        assert_eq!(creds.verifying_key(), signing_key.verifying_key());
    }

    async fn spawn_test_node() -> PandaNode {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("node.sqlite");
        std::mem::forget(dir); // keep the tempdir alive for the node's lifetime

        let params = RequiredNodeParams {
            credentials: p2panda::Credentials::generate(),
            network_id: Hash::from_bytes([0u8; 32]),
            bootstrap_node_ids: vec![],
            relay_url: None,
        };
        PandaNode::new(&params, db_path.to_str().unwrap()).await.unwrap()
    }

    async fn next_event(rx: &mut mpsc::Receiver<SubscriptionEvent>) -> SubscriptionEvent {
        tokio::time::timeout(Duration::from_secs(5), rx.recv())
            .await
            .expect("timed out waiting for subscription event")
            .expect("subscription channel closed unexpectedly")
    }

    async fn next_operation_bytes(rx: &mut mpsc::Receiver<SubscriptionEvent>) -> Vec<u8> {
        match next_event(rx).await {
            SubscriptionEvent::Operation(op) => op.bytes,
            _ => panic!("expected an operation event, got a lifecycle event instead"),
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn primary_subscription_receives_own_published_operations() {
        let node = spawn_test_node().await;
        let topic = RegionAppTopic::new(RegionId::generate(), "test-app");

        let (primary_tx, mut primary_rx) = mpsc::channel::<IncomingOperation>(32);
        node.subscribe_to_region_topic(&topic, primary_tx).await.unwrap();

        node.publish_to_region_topic(&topic, b"first".to_vec()).await.unwrap();

        let received = tokio::time::timeout(Duration::from_secs(10), primary_rx.recv())
            .await
            .expect("timed out waiting for own published operation");
        assert_eq!(received.unwrap().bytes, b"first");
    }

    /// Two named-cursor replays of the same topic run independently from the
    /// start, without interfering with each other's ack state.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn named_cursor_replays_are_independent() {
        let node = spawn_test_node().await;
        let topic = RegionAppTopic::new(RegionId::generate(), "test-app");

        // The primary subscription is required before publishing works.
        let (primary_tx, mut primary_rx) = mpsc::channel::<IncomingOperation>(32);
        node.subscribe_to_region_topic(&topic, primary_tx).await.unwrap();

        node.publish_to_region_topic(&topic, b"first".to_vec()).await.unwrap();
        node.publish_to_region_topic(&topic, b"second".to_vec()).await.unwrap();

        // Drain the primary subscription's copies so they don't pile up.
        tokio::time::timeout(Duration::from_secs(5), primary_rx.recv()).await.unwrap();
        tokio::time::timeout(Duration::from_secs(5), primary_rx.recv()).await.unwrap();

        // Instance "a" replays from the start under its own cursor name.
        let (tx_a, mut rx_a) = mpsc::channel::<SubscriptionEvent>(32);
        node.replay_region_topic_as(&topic, "instance-a", tx_a).await.unwrap();

        assert!(matches!(
            next_event(&mut rx_a).await,
            SubscriptionEvent::ReplayStarted { total_operations: 2 }
        ));
        assert_eq!(next_operation_bytes(&mut rx_a).await, b"first");
        assert_eq!(next_operation_bytes(&mut rx_a).await, b"second");
        assert!(matches!(next_event(&mut rx_a).await, SubscriptionEvent::ReplayEnded));

        // Instance "b" independently replays the same history under a
        // different cursor name, unaffected by instance "a"'s cursor.
        let (tx_b, mut rx_b) = mpsc::channel::<SubscriptionEvent>(32);
        node.replay_region_topic_as(&topic, "instance-b", tx_b).await.unwrap();

        assert!(matches!(
            next_event(&mut rx_b).await,
            SubscriptionEvent::ReplayStarted { total_operations: 2 }
        ));
        assert_eq!(next_operation_bytes(&mut rx_b).await, b"first");
        assert_eq!(next_operation_bytes(&mut rx_b).await, b"second");
        assert!(matches!(next_event(&mut rx_b).await, SubscriptionEvent::ReplayEnded));
    }

    /// The replay task stops right after `ReplayEnded` — a live operation
    /// published afterwards must never reach it, and the channel must close
    /// (not just go quiet) to prove the task actually ended.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn named_cursor_replay_stops_after_replay_ended() {
        let node = spawn_test_node().await;
        let topic = RegionAppTopic::new(RegionId::generate(), "test-app");

        let (primary_tx, mut primary_rx) = mpsc::channel::<IncomingOperation>(32);
        node.subscribe_to_region_topic(&topic, primary_tx).await.unwrap();

        node.publish_to_region_topic(&topic, b"first".to_vec()).await.unwrap();
        tokio::time::timeout(Duration::from_secs(5), primary_rx.recv()).await.unwrap();

        let (tx, mut rx) = mpsc::channel::<SubscriptionEvent>(32);
        node.replay_region_topic_as(&topic, "instance-a", tx).await.unwrap();

        assert!(matches!(
            next_event(&mut rx).await,
            SubscriptionEvent::ReplayStarted { total_operations: 1 }
        ));
        assert_eq!(next_operation_bytes(&mut rx).await, b"first");
        assert!(matches!(next_event(&mut rx).await, SubscriptionEvent::ReplayEnded));

        // Published after replay ended: a still-running subscription would
        // deliver this as a live operation.
        node.publish_to_region_topic(&topic, b"second".to_vec()).await.unwrap();

        let event = tokio::time::timeout(Duration::from_secs(5), rx.recv())
            .await
            .expect("recv should return promptly once the subscription task has ended");
        assert!(event.is_none(), "expected the channel to be closed, but got another event");
    }
}
