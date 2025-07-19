use crate::{config::LoresNodeConfig, repos::helpers::NETWORK_CONFIG_ID};
use hex;
use p2panda_core::{identity::PRIVATE_KEY_LEN, PrivateKey};
use sqlx::SqlitePool;

pub struct ThisP2PandaNodeRepo {}

#[derive(Clone)]
pub struct SimplifiedNodeAddress {
    pub node_id: String,
}

impl ThisP2PandaNodeRepo {
    pub fn init() -> Self {
        ThisP2PandaNodeRepo {}
    }

    pub async fn get_network_name(&self, pool: &SqlitePool) -> Result<Option<String>, sqlx::Error> {
        let result = sqlx::query!(
            "
            SELECT network_name
            FROM network_configs
            WHERE network_configs.id = ?
            LIMIT 1
            ",
            NETWORK_CONFIG_ID
        )
        .fetch_optional(pool)
        .await?;

        match result {
            None => return Ok(None),
            Some(result) => return Ok(result.network_name),
        }
    }

    pub async fn get_bootstrap_details(
        &self,
        pool: &SqlitePool,
    ) -> Result<Option<SimplifiedNodeAddress>, sqlx::Error> {
        let result = sqlx::query!(
            "
            SELECT bootstrap_node_id
            FROM network_configs
            WHERE network_configs.id = ?
            LIMIT 1
            ",
            NETWORK_CONFIG_ID
        )
        .fetch_optional(pool)
        .await?;

        match result {
            None => return Ok(None),
            Some(result) => match result.bootstrap_node_id {
                None => return Ok(None),
                Some(node_id) => Ok(Some(SimplifiedNodeAddress { node_id })),
            },
        }
    }

    pub async fn set_network_config(
        &self,
        pool: &SqlitePool,
        network_name: String,
        peer_address: Option<SimplifiedNodeAddress>,
    ) -> Result<(), sqlx::Error> {
        let bootstrap_node_id = peer_address.as_ref().map(|peer| peer.node_id.clone());

        let _region = sqlx::query!(
            "
            UPDATE network_configs
            SET network_name = ?, bootstrap_node_id = ?
            WHERE network_configs.id = ?
            ",
            network_name,
            bootstrap_node_id,
            NETWORK_CONFIG_ID
        )
        .execute(pool)
        .await;

        return Ok(());
    }

    pub async fn get_or_create_private_key(
        &self,
        config: &mut LoresNodeConfig,
    ) -> Result<PrivateKey, sqlx::Error> {
        let private_key = self.get_private_key(config).await?;

        match private_key {
            None => {
                let new_private_key: PrivateKey = self.create_private_key(config).await?;
                return Ok(new_private_key);
            }
            Some(private_key) => {
                return Ok(private_key);
            }
        }
    }

    async fn get_private_key(
        &self,
        config: &LoresNodeConfig,
    ) -> Result<Option<PrivateKey>, sqlx::Error> {
        let private_key_hex: Option<String> = config.private_key_hex.clone();

        match private_key_hex {
            None => return Ok(None),
            Some(private_key_hex) => {
                let private_key = Self::build_private_key_from_hex(private_key_hex)
                    .ok_or_else(|| sqlx::Error::Decode("Failed to build private key".into()))?;

                return Ok(Some(private_key));
            }
        }
    }

    async fn create_private_key(
        &self,
        config: &mut LoresNodeConfig,
    ) -> Result<PrivateKey, sqlx::Error> {
        let new_private_key = PrivateKey::new();
        let public_key = new_private_key.public_key();

        self.set_private_key_hex(config, new_private_key.to_hex(), public_key.to_hex())
            .await?;

        println!("Created new private key");

        return Ok(new_private_key);
    }

    async fn set_private_key_hex(
        &self,
        config: &mut LoresNodeConfig,
        private_key_hex: String,
        public_key_hex: String,
    ) -> Result<(), sqlx::Error> {
        config.public_key_hex = Some(public_key_hex.clone());
        config.private_key_hex = Some(private_key_hex.clone());
        config.save();

        Ok(())
    }

    // TODO: This should be in p2panda-core, submit a PR
    fn build_private_key_from_hex(private_key_hex: String) -> Option<PrivateKey> {
        let private_key_bytes = hex::decode(private_key_hex).ok()?;
        let private_key_byte_array: [u8; PRIVATE_KEY_LEN] = private_key_bytes.try_into().ok()?;
        Some(PrivateKey::from_bytes(&private_key_byte_array))
    }
}
