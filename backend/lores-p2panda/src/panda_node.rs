use std::collections::HashMap;
use std::sync::LazyLock;

use p2panda::Node;
use p2panda::node::SpawnError;
use p2panda::streams::{Offset, PublishError, StreamEvent, StreamPublisher};
use p2panda_core::{Hash, PrivateKey, PublicKey, Topic};
use p2panda_store::SqliteError;
use p2panda_net::iroh_endpoint::RelayUrl;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::Row;
use thiserror::Error;
use tokio::sync::{RwLock, mpsc};
use tokio_stream::StreamExt;

use crate::IncomingOperation;

static RELAY_URL: LazyLock<RelayUrl> = LazyLock::new(|| {
    "https://euc1-1.relay.n0.iroh-canary.iroh.link"
        .parse()
        .expect("valid relay URL")
});

static BOOTSTRAP_NODE_ID: LazyLock<PublicKey> = LazyLock::new(|| {
    "9f63a15ab95959a992af96bf72fbc3e7dc98eeb4799f788bb07b20125053e795"
        .parse()
        .expect("valid bootstrap node id")
});

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
    pub private_key: PrivateKey,
    pub network_id: Hash,
    pub bootstrap_node_ids: Vec<PublicKey>,
}

pub struct PandaNode {
    network: RwLock<Node>,
    publishers: RwLock<HashMap<Topic, StreamPublisher<Vec<u8>>>>,
    pool: SqlitePool,
    pub public_key: PublicKey,
}

impl PandaNode {
    pub async fn new(
        params: &RequiredNodeParams,
        database_url: &str,
    ) -> Result<Self, PandaNodeError> {
        let public_key = params.private_key.public_key();
        let network_id: [u8; 32] = *params.network_id.as_bytes();

        let mut builder = Node::builder()
            .network_id(network_id)
            .private_key(params.private_key.clone())
            .database_url(database_url);

        if cfg!(not(test)) {
            for bootstrap_id in &params.bootstrap_node_ids {
                builder = builder.bootstrap(*bootstrap_id, RELAY_URL.clone());
            }
            builder = builder
                .bootstrap(*BOOTSTRAP_NODE_ID, RELAY_URL.clone())
                .relay_url(RELAY_URL.clone());
        }

        let node = builder.spawn().await?;

        // Open a read-only pool against the same file for diagnostic queries.
        let pool = open_pool(database_url).await?;

        Ok(Self {
            network: RwLock::new(node),
            publishers: RwLock::new(HashMap::new()),
            pool,
            public_key,
        })
    }

    pub async fn subscribe_to_topic(
        &self,
        topic_id: Topic,
        events_tx: mpsc::Sender<IncomingOperation>,
    ) -> Result<(), SubscriptionError> {
        if self.publishers.read().await.contains_key(&topic_id) {
            return Err(SubscriptionError::AlreadySubscribed(topic_id));
        }

        let network = self.network.read().await;
        let (publisher, mut subscription) = network
            .stream_from::<Vec<u8>>(topic_id, Offset::Frontier)
            .await?;
        drop(network);

        self.publishers.write().await.insert(topic_id, publisher);

        tokio::spawn(async move {
            while let Some(event) = subscription.next().await {
                match event {
                    StreamEvent::Processed(op) => {
                        let incoming = IncomingOperation {
                            author: op.author(),
                            topic: op.topic(),
                            bytes: op.message().clone(),
                            operation_id: op.id(),
                            timestamp: op.timestamp(),
                        };
                        if events_tx.send(incoming).await.is_err() {
                            break;
                        }
                    }
                    StreamEvent::DecodingFailed { error, .. } => {
                        tracing::error!("failed decoding incoming operation: {error}");
                    }
                    StreamEvent::ReplayFailed { error, .. } => {
                        tracing::error!("error replaying operation stream: {error}");
                    }
                    StreamEvent::SyncStarted { .. } | StreamEvent::SyncEnded { .. } => {}
                }
            }
        });

        Ok(())
    }

    pub async fn publish(&self, topic_id: Topic, bytes: Vec<u8>) -> Result<(), PandaPublishError> {
        let publishers = self.publishers.read().await;
        let publisher = publishers
            .get(&topic_id)
            .ok_or(PandaPublishError::NoSubscription(topic_id))?;
        publisher.publish(bytes).await?;
        Ok(())
    }

    pub async fn get_log_counts(&self) -> Result<Vec<LogCount>, SqliteError> {
        let rows = sqlx::query(
            "SELECT public_key, COUNT(*) AS total FROM operations_v1 GROUP BY public_key",
        )
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
}

pub struct LogCount {
    pub node_id: String,
    pub total: i64,
}

async fn open_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // Strip the "sqlite:" scheme prefix if present since SqliteConnectOptions
    // wants just the path.
    let path = database_url
        .strip_prefix("sqlite:")
        .unwrap_or(database_url);

    let options = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);

    SqlitePoolOptions::new()
        .max_connections(4)
        .connect_with(options)
        .await
}
