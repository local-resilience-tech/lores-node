use p2panda_core::{PrivateKey, PublicKey};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::panda_node::PandaNode;

#[derive(Default, Clone)]
pub struct NodeParams {
    pub private_key: Option<PrivateKey>,
    pub network_name: Option<String>,
    pub bootstrap_node_id: Option<PublicKey>,
}

#[derive(Clone)]
pub struct PandaNodeContainer {
    params: Arc<Mutex<NodeParams>>,
    node: Arc<Mutex<Option<PandaNode>>>,
}

impl PandaNodeContainer {
    pub fn new() -> Self {
        let params = Arc::new(Mutex::new(NodeParams::default()));

        PandaNodeContainer {
            params,
            node: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn get_params(&self) -> NodeParams {
        let params_lock = self.params.lock().await;
        params_lock.clone()
    }

    pub async fn set_network_name(&self, network_name: String) {
        let mut params_lock = self.params.lock().await;
        params_lock.network_name = Some(network_name);
    }

    pub async fn set_private_key(&self, private_key: PrivateKey) {
        let mut params_lock = self.params.lock().await;
        params_lock.private_key = Some(private_key);
    }

    pub async fn set_bootstrap_node_id(&self, bootstrap_node_id: Option<PublicKey>) {
        let mut params_lock = self.params.lock().await;
        params_lock.bootstrap_node_id = bootstrap_node_id;
    }
}
