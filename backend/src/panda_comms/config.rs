use crate::config::config_state::LoresNodeConfigState;
use hex;
use p2panda_core::{identity::PRIVATE_KEY_LEN, PrivateKey};

#[derive(Clone)]
pub struct SimplifiedNodeAddress {
    pub node_id: String,
}

pub struct ThisP2PandaNodeRepo {}

impl ThisP2PandaNodeRepo {
    pub fn init() -> Self {
        ThisP2PandaNodeRepo {}
    }

    pub async fn get_bootstrap_details(
        &self,
        config_state: &LoresNodeConfigState,
    ) -> Option<SimplifiedNodeAddress> {
        let config = config_state.get().await;

        config
            .bootstrap_node_id
            .clone()
            .map(|bootstrap_node_id| SimplifiedNodeAddress {
                node_id: bootstrap_node_id,
            })
    }

    pub async fn set_network_config(
        &self,
        config_state: &LoresNodeConfigState,
        peer_address: Option<SimplifiedNodeAddress>,
    ) -> Result<(), anyhow::Error> {
        config_state
            .update(|config| {
                let mut result = config.clone();
                result.bootstrap_node_id = peer_address.as_ref().map(|peer| peer.node_id.clone());
                result
            })
            .await
    }

    pub async fn get_or_create_private_key(
        &self,
        config_state: &LoresNodeConfigState,
    ) -> Result<PrivateKey, anyhow::Error> {
        let private_key = self.get_private_key(config_state).await;

        match private_key {
            None => self.create_private_key(config_state).await,
            Some(private_key) => Ok(private_key),
        }
    }

    async fn get_private_key(&self, config_state: &LoresNodeConfigState) -> Option<PrivateKey> {
        let config = config_state.get().await;

        config
            .private_key_hex
            .clone()
            .map(|hex| Self::build_private_key_from_hex(hex))
            .flatten()
    }

    async fn create_private_key(
        &self,
        config_state: &LoresNodeConfigState,
    ) -> Result<PrivateKey, anyhow::Error> {
        let new_private_key = PrivateKey::new();
        let public_key = new_private_key.public_key();

        self.set_private_key_hex(config_state, new_private_key.to_hex(), public_key.to_hex())
            .await?;

        println!("Created new private key");
        Ok(new_private_key)
    }

    async fn set_private_key_hex(
        &self,
        config_state: &LoresNodeConfigState,
        private_key_hex: String,
        public_key_hex: String,
    ) -> Result<(), anyhow::Error> {
        config_state
            .update(|config| {
                let mut result = config.clone();
                result.private_key_hex = Some(private_key_hex);
                result.public_key_hex = Some(public_key_hex);
                result
            })
            .await
    }

    // TODO: This should be in p2panda-core, submit a PR
    fn build_private_key_from_hex(private_key_hex: String) -> Option<PrivateKey> {
        let private_key_bytes = hex::decode(private_key_hex).ok()?;
        let private_key_byte_array: [u8; PRIVATE_KEY_LEN] = private_key_bytes.try_into().ok()?;
        Some(PrivateKey::from_bytes(&private_key_byte_array))
    }
}
