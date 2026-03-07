use crate::{config::config_state::LoresNodeConfigState, panda_comms::build_public_key_from_hex};
use hex;
use lores_p2panda::p2panda_core::{identity::PRIVATE_KEY_LEN, PrivateKey, PublicKey};

pub struct ThisP2PandaNodeRepo {}

impl ThisP2PandaNodeRepo {
    pub fn init() -> Self {
        ThisP2PandaNodeRepo {}
    }

    pub async fn get_bootstrap_node_ids(
        &self,
        config_state: &LoresNodeConfigState,
    ) -> Vec<PublicKey> {
        let config = config_state.get().await;

        config
            .bootstrap_node_ids
            .unwrap_or_default()
            .into_iter()
            .filter_map(|bootstrap_node_id| build_public_key_from_hex(&bootstrap_node_id).ok())
            .collect()
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
