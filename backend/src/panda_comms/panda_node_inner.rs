use p2panda_core::{Hash, PrivateKey, PublicKey};
use tokio::sync::RwLock;

use super::{network::Network, panda_node::PandaNodeError};

#[allow(dead_code)]
pub struct PandaNodeInner {
    network: RwLock<Network>,
}

impl PandaNodeInner {
    pub async fn new(
        network_id: Hash,
        private_key: PrivateKey,
        bootstrap_node_id: Option<PublicKey>,
    ) -> Result<Self, PandaNodeError> {
        println!("Initializing PandaNodeInner...");

        let network = Network::new(network_id, private_key, bootstrap_node_id).await?;

        Ok(PandaNodeInner {
            network: RwLock::new(network),
        })
    }
}
