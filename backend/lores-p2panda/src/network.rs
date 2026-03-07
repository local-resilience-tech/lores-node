use p2panda_core::{Hash, PrivateKey, PublicKey};
use p2panda_net::{
    address_book::AddressBookError,
    addrs::NodeInfo,
    discovery::DiscoveryError,
    gossip::GossipError,
    iroh_endpoint::EndpointError,
    iroh_mdns::{MdnsDiscoveryError, MdnsDiscoveryMode},
    AddressBook, Discovery, Endpoint, Gossip, MdnsDiscovery,
};
use thiserror::Error;

use super::{
    operation_store::OperationStore,
    operations::LoResMeshExtensions,
    topic::{LoResNodeTopicMap, LogId},
};

lazy_static! {
    pub static ref RELAY_URL: iroh::RelayUrl = "https://euc1-1.relay.n0.iroh-canary.iroh.link"
        .parse()
        .expect("valid relay URL");
}

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
}

#[allow(dead_code)]
pub struct Network {
    address_book: AddressBook,
    mdns_discovery: MdnsDiscovery,
    discovery: Discovery,
    gossip: Gossip,
    log_sync: LogSync,
    endpoint: Endpoint,
    pub topic_map: LoResNodeTopicMap,
}

impl Network {
    pub async fn new(
        network_id: Hash,
        private_key: PrivateKey,
        bootstrap_node_ids: &Vec<PublicKey>,
        operation_store: &OperationStore,
    ) -> Result<Self, NetworkError> {
        println!("Initializing P2Panda Network (id: {})", network_id.to_hex());

        let address_book = AddressBook::builder().spawn().await?;

        for bootstrap_node_id in bootstrap_node_ids.iter() {
            Self::add_bootstrap_node_to_address_book(bootstrap_node_id, &address_book).await?;
        }

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

        let log_sync = LogSync::builder(
            operation_store.clone_inner(),
            topic_map.clone(),
            endpoint.clone(),
            gossip.clone(),
        )
        .spawn()
        .await?;

        Ok(Network {
            address_book,
            mdns_discovery,
            discovery,
            gossip,
            log_sync,
            endpoint,
            topic_map,
        })
    }

    pub fn get_log_sync(&self) -> &LogSync {
        &self.log_sync
    }

    pub async fn add_bootstrap_node(
        &self,
        bootstrap_node_id: &PublicKey,
    ) -> Result<(), NetworkError> {
        Self::add_bootstrap_node_to_address_book(bootstrap_node_id, &self.address_book).await
    }

    async fn add_bootstrap_node_to_address_book(
        bootstrap_node_id: &PublicKey,
        address_book: &AddressBook,
    ) -> Result<(), NetworkError> {
        let bootstrap_info = bootstrap_node_info(bootstrap_node_id);
        println!(
            "Adding bootstrap node info for node: {:?}",
            bootstrap_info.node_id.to_hex()
        );
        address_book.insert_node_info(bootstrap_info).await?;
        Ok(())
    }
}

fn bootstrap_node_info(bootstrap_node_id: &PublicKey) -> NodeInfo {
    let endpoint_addr = iroh::EndpointAddr::new(
        bootstrap_node_id
            .to_hex()
            .parse()
            .expect("valid bootstrap node id"),
    )
    .with_relay_url(RELAY_URL.clone());
    NodeInfo::from(endpoint_addr).bootstrap()
}
