use p2panda_core::{Hash, PrivateKey, PublicKey};
use p2panda_net::{
    address_book::AddressBookError,
    addrs::NodeInfo,
    gossip::GossipError,
    iroh_endpoint::EndpointError,
    iroh_mdns::{MdnsDiscoveryError, MdnsDiscoveryMode},
    AddressBook, Endpoint, Gossip, MdnsDiscovery,
};
use thiserror::Error;

lazy_static! {
    pub static ref RELAY_URL: iroh::RelayUrl = "https://euc1-1.relay.n0.iroh-canary.iroh.link"
        .parse()
        .expect("valid relay URL");
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error(transparent)]
    AddressBook(#[from] AddressBookError),
    #[error(transparent)]
    Endpoint(#[from] EndpointError),
    #[error(transparent)]
    MdnsDiscovery(#[from] MdnsDiscoveryError),
    #[error(transparent)]
    Gossip(#[from] GossipError),
}

#[allow(dead_code)]
pub struct Network {
    endpoint: Endpoint,
}

impl Network {
    pub async fn new(
        network_id: Hash,
        private_key: PrivateKey,
        bootstrap_node_id: Option<PublicKey>,
    ) -> Result<Self, NetworkError> {
        println!("Initializing P2Panda Network...");

        let address_book = AddressBook::builder().spawn().await?;
        if let Some(bootstrap_info) = bootstrap_node_info(bootstrap_node_id) {
            if let Err(e) = address_book.insert_node_info(bootstrap_info).await {
                println!("Failed to insert bootstrap node info: {}", e);
            }
        }

        let endpoint = Endpoint::builder(address_book.clone())
            .network_id(network_id.into())
            .private_key(private_key.clone())
            .relay_url(RELAY_URL.clone())
            .spawn()
            .await?;

        MdnsDiscovery::builder(address_book.clone(), endpoint.clone())
            .mode(MdnsDiscoveryMode::Active)
            .spawn()
            .await?;

        Gossip::builder(address_book.clone(), endpoint.clone())
            .spawn()
            .await?;

        Ok(Network { endpoint })
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
