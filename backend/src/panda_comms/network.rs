use p2panda_core::{Hash, Operation, PrivateKey, PublicKey};
use p2panda_net::{
    address_book::AddressBookError,
    addrs::NodeInfo,
    discovery::DiscoveryError,
    gossip::GossipError,
    iroh_endpoint::EndpointError,
    iroh_mdns::{MdnsDiscoveryError, MdnsDiscoveryMode},
    sync::{SyncHandle, SyncHandleError},
    AddressBook, Discovery, Endpoint, Gossip, MdnsDiscovery, TopicId,
};
use p2panda_sync::protocols::TopicLogSyncEvent;
use thiserror::Error;

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

#[allow(dead_code)]
pub struct Network {
    address_book: AddressBook,
    mdns_discovery: MdnsDiscovery,
    discovery: Discovery,
    gossip: Gossip,
    log_sync: LogSync,
    endpoint: Endpoint,
    sync_tx: SyncHandle<Operation<LoResMeshExtensions>, TopicLogSyncEvent<LoResMeshExtensions>>,
}

impl Network {
    pub async fn new(
        network_id: Hash,
        private_key: PrivateKey,
        _bootstrap_node_id: Option<PublicKey>,
        operation_store: &OperationStore,
    ) -> Result<Self, NetworkError> {
        println!("Initializing P2Panda Network (id: {})", network_id.to_hex());

        let address_book = AddressBook::builder().spawn().await?;

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

        let endpoint = Endpoint::builder(address_book.clone())
            .network_id(network_id.into())
            .private_key(private_key.clone())
            .relay_url(RELAY_URL.clone())
            .spawn()
            .await?;

        let mdns_discovery = MdnsDiscovery::builder(address_book.clone(), endpoint.clone())
            .mode(MdnsDiscoveryMode::Active)
            .spawn()
            .await?;

        let discovery = Discovery::builder(address_book.clone(), endpoint.clone())
            .spawn()
            .await?;

        let gossip = Gossip::builder(address_book.clone(), endpoint.clone())
            .spawn()
            .await?;

        let topic_map = LoResNodeTopicMap::default();
        topic_map
            .insert(NODE_ADMIN_TOPIC_ID, private_key.public_key(), LOG_ID)
            .await;

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

        let log_sync = LogSync::builder(
            operation_store.clone_inner(),
            topic_map.clone(),
            endpoint.clone(),
            gossip.clone(),
        )
        .spawn()
        .await?;

        let sync_tx = log_sync.stream(NODE_ADMIN_TOPIC_ID, true).await?;

        Ok(Network {
            address_book,
            mdns_discovery,
            discovery,
            gossip,
            log_sync,
            endpoint,
            sync_tx,
        })
    }

    pub async fn publish_operation(
        &self,
        operation: Operation<LoResMeshExtensions>,
    ) -> Result<
        (),
        SyncHandleError<Operation<LoResMeshExtensions>, TopicLogSyncEvent<LoResMeshExtensions>>,
    > {
        println!(
            "Publishing operation to LogSync: {:?}",
            operation.hash.to_hex()
        );
        self.sync_tx.publish(operation).await.map_err(|e| {
            println!("Error publishing operation: {:?}", e);
            e
        })?;
        Ok(())
    }

    pub fn get_sync_handle(
        &self,
    ) -> &SyncHandle<Operation<LoResMeshExtensions>, TopicLogSyncEvent<LoResMeshExtensions>> {
        &self.sync_tx
    }
}

// fn bootstrap_node_info(bootstrap_node_id: Option<PublicKey>) -> Option<NodeInfo> {
//     bootstrap_node_id.map(|node_id| {
//         let endpoint_addr =
//             iroh::EndpointAddr::new(node_id.to_hex().parse().expect("valid bootstrap node id"))
//                 .with_relay_url(RELAY_URL.clone());
//         NodeInfo::from(endpoint_addr).bootstrap()
//     })
// }
