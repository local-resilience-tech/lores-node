use std::{collections::HashMap, sync::Arc};

use futures_util::StreamExt;
use p2panda_core::{Hash, Operation, PrivateKey, PublicKey};
use p2panda_net::{
    address_book::AddressBookError,
    addrs::NodeInfo,
    discovery::DiscoveryError,
    gossip::GossipError,
    iroh_endpoint::EndpointError,
    iroh_mdns::{MdnsDiscoveryError, MdnsDiscoveryMode},
    sync::{SyncHandle, SyncHandleError},
    utils::ShortFormat,
    AddressBook, Discovery, Endpoint, Gossip, MdnsDiscovery, NodeId, TopicId,
};
use p2panda_sync::{
    protocols::{Logs, TopicLogSyncEvent},
    traits::TopicMap,
};
use std::convert::Infallible;
use thiserror::Error;
use tokio::sync::RwLock;

use super::{
    operation_store::{OperationStore, LOG_ID},
    operations::LoResMeshExtensions,
    topic::{LoResNodeTopicMap, LogId},
};

lazy_static! {
    pub static ref RELAY_URL: iroh::RelayUrl = "https://euc1-1.relay.n0.iroh-canary.iroh.link"
        .parse()
        .expect("valid relay URL");
}

pub const NODE_ADMIN_TOPIC_ID: TopicId = [0u8; 32];

pub type LogSync = p2panda_net::sync::LogSync<
    p2panda_store::SqliteStore<LogId, LoResMeshExtensions>,
    LogId,
    LoResMeshExtensions,
    LoResNodeTopicMap,
>;
pub type LogSyncError = p2panda_net::sync::LogSyncError<LoResMeshExtensions>;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error(transparent)]
    AddressBook(#[from] AddressBookError),
    #[error(transparent)]
    Endpoint(#[from] EndpointError),
    #[error(transparent)]
    MdnsDiscovery(#[from] MdnsDiscoveryError),
    #[error(transparent)]
    Discovery(#[from] DiscoveryError),
    #[error(transparent)]
    Gossip(#[from] GossipError),
    #[error(transparent)]
    LogSync(#[from] LogSyncError),
    #[error("LogSync stream error: {0}")]
    SyncHandleError(String),
}

#[derive(Clone, Default, Debug)]
pub struct ChatTopicMap(Arc<RwLock<HashMap<TopicId, Logs<LogId>>>>);

impl ChatTopicMap {
    async fn insert(&self, topic_id: TopicId, node_id: NodeId, log_id: LogId) {
        let mut map = self.0.write().await;
        map.entry(topic_id)
            .and_modify(|logs| {
                logs.insert(node_id, vec![log_id]);
            })
            .or_insert({
                let mut value = HashMap::new();
                value.insert(node_id, vec![log_id]);
                value
            });
    }
}

impl TopicMap<TopicId, Logs<LogId>> for ChatTopicMap {
    type Error = Infallible;

    async fn get(&self, topic_query: &TopicId) -> Result<Logs<LogId>, Self::Error> {
        let map = self.0.read().await;
        Ok(map.get(topic_query).cloned().unwrap_or_default())
    }
}

#[allow(dead_code)]
pub struct Network {
    // endpoint: Endpoint,
    // log_sync: LogSync,
    // sync_tx: SyncHandle<Operation<LoResMeshExtensions>, TopicLogSyncEvent<LoResMeshExtensions>>,
    topic_map: ChatTopicMap,
    address_book: AddressBook,
    endpoint: Endpoint,
    discovery: Discovery,
    mdns: MdnsDiscovery,
}

impl Network {
    pub async fn new(
        network_id: Hash,
        private_key: PrivateKey,
        bootstrap_node_id: Option<PublicKey>,
        operation_store: &OperationStore,
    ) -> Result<Self, NetworkError> {
        println!("Initializing P2Panda Network (id: {})", network_id.to_hex());

        // CHAT APP CODE

        let private_key = PrivateKey::new();
        let public_key = private_key.public_key();
        let topic_id: TopicId = Hash::new(b"lores-mesh").into();

        let topic_map = ChatTopicMap::default();
        topic_map.insert(topic_id, public_key, LOG_ID).await;

        let address_book = AddressBook::builder().spawn().await?;

        let network_id = Hash::new("merri-bek".as_bytes());

        let endpoint = Endpoint::builder(address_book.clone())
            .private_key(private_key.clone())
            .network_id(network_id.into())
            .relay_url(RELAY_URL.clone())
            .spawn()
            .await?;

        println!("network id: {}", endpoint.network_id().fmt_short());
        println!("chat topic id: {}", hex::encode(topic_id));
        println!("public key: {}", public_key.to_hex());
        println!("relay url: {}", RELAY_URL.to_string());

        let discovery = Discovery::builder(address_book.clone(), endpoint.clone())
            .spawn()
            .await?;

        let mdns = MdnsDiscovery::builder(address_book.clone(), endpoint.clone())
            .mode(MdnsDiscoveryMode::Active)
            .spawn()
            .await?;

        println!("P2Panda Network initialized.");

        Ok(Network {
            topic_map,
            address_book,
            endpoint,
            discovery,
            mdns,
        })

        // OLD CODE

        // let address_book = AddressBook::builder().spawn().await?;

        // if let Some(bootstrap_info) = bootstrap_node_info(bootstrap_node_id) {
        //     println!(
        //         "Inserting bootstrap node info for node: {:?}",
        //         bootstrap_info.node_id.to_hex()
        //     );
        //     if let Err(e) = address_book.insert_node_info(bootstrap_info).await {
        //         println!("Failed to insert bootstrap node info: {}", e);
        //     }
        // }

        // let mut topic_rx = address_book.watch_topic(NODE_ADMIN_TOPIC_ID, false).await?;

        // // Subscribe to topic updates
        // {
        //     tokio::spawn(async move {
        //         while let Some(update) = topic_rx.recv().await {
        //             let update_hexes = match &update.difference {
        //                 Some(diff) => diff.iter().map(|h| h.to_hex()).collect::<Vec<_>>(),
        //                 None => vec![],
        //             };
        //             let value_hexes = update.value.iter().map(|h| h.to_hex()).collect::<Vec<_>>();
        //             println!(
        //                 "  AddressBook topic update: diff {:?}, value {:?}",
        //                 update_hexes, value_hexes
        //             );
        //         }
        //     });
        // }

        // let endpoint = Endpoint::builder(address_book.clone())
        //     .network_id(network_id.into())
        //     .private_key(private_key.clone())
        //     .relay_url(RELAY_URL.clone())
        //     .spawn()
        //     .await?;

        // let discovery = Discovery::builder(address_book.clone(), endpoint.clone())
        //     .spawn()
        //     .await?;

        // MdnsDiscovery::builder(address_book.clone(), endpoint.clone())
        //     .mode(MdnsDiscoveryMode::Active)
        //     .spawn()
        //     .await?;

        // let gossip = Gossip::builder(address_book.clone(), endpoint.clone())
        //     .spawn()
        //     .await?;

        // let topic_map = LoResNodeTopicMap::default();
        // topic_map
        //     .insert(NODE_ADMIN_TOPIC_ID, private_key.public_key(), LOG_ID)
        //     .await;

        // // Subscribe to discovery events
        // let mut discovery_events_rx = discovery.events().await?;
        // {
        //     tokio::spawn(async move {
        //         while let Ok(event) = discovery_events_rx.recv().await {
        //             println!("  Discovery event: {:?}", event);
        //         }
        //     });
        // }

        // let gossip_tx = gossip.stream(NODE_ADMIN_TOPIC_ID).await?;
        // let mut gossip_rx = gossip_tx.subscribe();
        // // Receive and log each (ephemeral) message.
        // {
        //     tokio::spawn(async move {
        //         loop {
        //             if let Some(Ok(message)) = gossip_rx.next().await {
        //                 println!("  received gossip message: {:?}", message);
        //             }
        //         }
        //     });
        // }

        // let log_sync = LogSync::builder(operation_store.clone_inner(), topic_map, endpoint, gossip)
        //     .spawn()
        //     .await?;

        // let sync_tx = log_sync.stream(NODE_ADMIN_TOPIC_ID, true).await?;
        // let mut sync_rx = sync_tx.subscribe().await.map_err(|e| {
        //     NetworkError::SyncHandleError(format!("Failed to subscribe to log sync: {}", e))
        // })?;

        // // Receive messages from the sync stream.
        // {
        //     tokio::task::spawn(async move {
        //         println!("  P2Panda Network initialized, starting sync stream...");
        //         while let Some(Ok(from_sync)) = sync_rx.next().await {
        //             match from_sync.event {
        //                 TopicLogSyncEvent::SyncStarted(_) => {
        //                     println!(
        //                         "  started sync session with {}",
        //                         from_sync.remote.fmt_short()
        //                     );
        //                 }
        //                 TopicLogSyncEvent::SyncFinished(metrics) => {
        //                     println!(
        //                     "  finished sync session with {}, bytes received = {}, bytes sent = {}",
        //                     from_sync.remote.fmt_short(),
        //                     metrics.total_bytes_remote.unwrap_or_default(),
        //                     metrics.total_bytes_local.unwrap_or_default()
        //                 );
        //                 }
        //                 TopicLogSyncEvent::Operation(operation) => {
        //                     println!(
        //                         "  Received operation from {}: {:?}",
        //                         from_sync.remote.fmt_short(),
        //                         operation
        //                     );
        //                 }
        //                 _ => {
        //                     println!(
        //                         "  Unhandled sync event from {}: {:?}",
        //                         from_sync.remote.fmt_short(),
        //                         from_sync.event
        //                     );
        //                 }
        //             }
        //         }
        //         println!("  Sync stream read loop ended.");
        //     });
        // }

        // Ok(Network {
        //     // endpoint,
        //     // log_sync,
        //     // sync_tx,
        // })
    }

    pub async fn publish_operation(
        &self,
        operation: Operation<LoResMeshExtensions>,
    ) -> Result<
        (),
        SyncHandleError<Operation<LoResMeshExtensions>, TopicLogSyncEvent<LoResMeshExtensions>>,
    > {
        // println!(
        //     "Publishing operation to LogSync: {:?}",
        //     operation.hash.to_hex()
        // );
        // self.sync_tx.publish(operation).await.map_err(|e| {
        //     println!("Error publishing operation: {:?}", e);
        //     e
        // })?;
        Ok(())
    }
}

fn bootstrap_node_info(bootstrap_node_id: Option<PublicKey>) -> Option<NodeInfo> {
    bootstrap_node_id.map(|node_id| {
        let endpoint_addr =
            iroh::EndpointAddr::new(node_id.to_hex().parse().expect("valid bootstrap node id"))
                .with_relay_url(RELAY_URL.clone());
        NodeInfo::from(endpoint_addr).bootstrap()
    })
}
