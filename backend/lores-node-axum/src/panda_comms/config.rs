use crate::{config::LoresNodeConfigState, panda_comms::build_public_key_from_hex};
use lores_p2panda::{Credentials, credentials_from_seed};
use p2panda_core::{SigningKey, VerifyingKey, identity::SIGNING_KEY_LEN};
use rand::RngExt;
use tracing::info;

const IDENTITY_SECRET_SEED_LEN: usize = 32;

pub struct ThisP2PandaNodeRepo {}

impl ThisP2PandaNodeRepo {
    pub fn init() -> Self {
        ThisP2PandaNodeRepo {}
    }

    pub async fn get_bootstrap_node_ids(&self, config_state: &LoresNodeConfigState) -> Vec<VerifyingKey> {
        let config = config_state.get().await;

        config
            .bootstrap_node_ids
            .unwrap_or_default()
            .into_iter()
            .filter_map(|bootstrap_node_id| build_public_key_from_hex(&bootstrap_node_id).ok())
            .collect()
    }

    pub async fn get_or_create_credentials(&self, config_state: &LoresNodeConfigState) -> Result<Credentials, anyhow::Error> {
        let signing_key = self.get_or_create_private_key(config_state).await?;
        let identity_secret_seed = self.get_or_create_identity_secret_seed(config_state).await?;
        Ok(credentials_from_seed(signing_key, identity_secret_seed))
    }

    pub async fn get_or_create_private_key(&self, config_state: &LoresNodeConfigState) -> Result<SigningKey, anyhow::Error> {
        let private_key = self.get_private_key(config_state).await;

        match private_key {
            None => self.create_private_key(config_state).await,
            Some(private_key) => Ok(private_key),
        }
    }

    async fn get_private_key(&self, config_state: &LoresNodeConfigState) -> Option<SigningKey> {
        let config = config_state.get().await;

        config.private_key_hex.clone().and_then(Self::build_private_key_from_hex)
    }

    async fn create_private_key(&self, config_state: &LoresNodeConfigState) -> Result<SigningKey, anyhow::Error> {
        let new_private_key = SigningKey::generate();
        let public_key = new_private_key.verifying_key();

        self.set_private_key_hex(config_state, new_private_key.to_hex(), public_key.to_hex())
            .await?;

        info!("Created new private key");
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

    async fn get_or_create_identity_secret_seed(
        &self,
        config_state: &LoresNodeConfigState,
    ) -> Result<[u8; IDENTITY_SECRET_SEED_LEN], anyhow::Error> {
        let config = config_state.get().await;

        if let Some(seed_hex) = config.identity_secret_seed_hex
            && let Ok(seed) = Self::parse_identity_secret_seed(&seed_hex)
        {
            return Ok(seed);
        }

        let mut seed = [0u8; IDENTITY_SECRET_SEED_LEN];
        rand::rng().fill(&mut seed);

        self.set_identity_secret_seed_hex(config_state, hex::encode(seed)).await?;

        info!("Created new identity secret seed");
        Ok(seed)
    }

    async fn set_identity_secret_seed_hex(&self, config_state: &LoresNodeConfigState, seed_hex: String) -> Result<(), anyhow::Error> {
        config_state
            .update(|config| {
                let mut result = config.clone();
                result.identity_secret_seed_hex = Some(seed_hex);
                result
            })
            .await
    }

    fn parse_identity_secret_seed(seed_hex: &str) -> Result<[u8; IDENTITY_SECRET_SEED_LEN], anyhow::Error> {
        let bytes = hex::decode(seed_hex)?;
        let seed: [u8; IDENTITY_SECRET_SEED_LEN] = bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("identity secret seed must be {} bytes", IDENTITY_SECRET_SEED_LEN))?;
        Ok(seed)
    }

    // TODO: This should be in p2panda-core, submit a PR
    fn build_private_key_from_hex(private_key_hex: String) -> Option<SigningKey> {
        let private_key_bytes = hex::decode(private_key_hex).ok()?;
        let private_key_byte_array: [u8; SIGNING_KEY_LEN] = private_key_bytes.try_into().ok()?;
        Some(SigningKey::from_bytes(&private_key_byte_array))
    }
}
