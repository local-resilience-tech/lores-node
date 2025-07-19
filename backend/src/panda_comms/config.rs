use crate::config::LoresNodeConfig;
use hex;
use p2panda_core::{identity::PRIVATE_KEY_LEN, PrivateKey};

pub struct ThisP2PandaNodeRepo {}

#[derive(Clone)]
pub struct SimplifiedNodeAddress {
    pub node_id: String,
}

impl ThisP2PandaNodeRepo {
    pub fn init() -> Self {
        ThisP2PandaNodeRepo {}
    }

    pub fn get_bootstrap_details(&self, config: &LoresNodeConfig) -> Option<SimplifiedNodeAddress> {
        config
            .bootstrap_node_id
            .clone()
            .map(|bootstrap_node_id| SimplifiedNodeAddress {
                node_id: bootstrap_node_id,
            })
    }

    pub fn set_network_config(
        &self,
        config: &mut LoresNodeConfig,
        network_name: String,
        peer_address: Option<SimplifiedNodeAddress>,
    ) {
        config.bootstrap_node_id = peer_address.as_ref().map(|peer| peer.node_id.clone());
        config.network_name = Some(network_name.clone());
        config.save();
    }

    pub fn get_or_create_private_key(&self, config: &mut LoresNodeConfig) -> PrivateKey {
        let private_key = self.get_private_key(config);

        match private_key {
            None => self.create_private_key(config),
            Some(private_key) => private_key,
        }
    }

    fn get_private_key(&self, config: &LoresNodeConfig) -> Option<PrivateKey> {
        config
            .private_key_hex
            .clone()
            .map(|hex| Self::build_private_key_from_hex(hex))
            .flatten()
    }

    fn create_private_key(&self, config: &mut LoresNodeConfig) -> PrivateKey {
        let new_private_key = PrivateKey::new();
        let public_key = new_private_key.public_key();

        self.set_private_key_hex(config, new_private_key.to_hex(), public_key.to_hex());

        println!("Created new private key");

        return new_private_key;
    }

    fn set_private_key_hex(
        &self,
        config: &mut LoresNodeConfig,
        private_key_hex: String,
        public_key_hex: String,
    ) {
        config.public_key_hex = Some(public_key_hex.clone());
        config.private_key_hex = Some(private_key_hex.clone());
        config.save();
    }

    // TODO: This should be in p2panda-core, submit a PR
    fn build_private_key_from_hex(private_key_hex: String) -> Option<PrivateKey> {
        let private_key_bytes = hex::decode(private_key_hex).ok()?;
        let private_key_byte_array: [u8; PRIVATE_KEY_LEN] = private_key_bytes.try_into().ok()?;
        Some(PrivateKey::from_bytes(&private_key_byte_array))
    }
}
