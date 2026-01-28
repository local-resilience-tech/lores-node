use p2panda_core::{Hash, PrivateKey};

use super::panda_node::PandaNodeError;

pub struct PandaNodeInner {}

impl PandaNodeInner {
    pub async fn new(network_id: Hash, private_key: PrivateKey) -> Result<Self, PandaNodeError> {
        println!("Initializing PandaNodeInner...");
        Ok(PandaNodeInner {})
    }
}
