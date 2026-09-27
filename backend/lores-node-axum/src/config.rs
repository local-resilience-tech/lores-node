use std::sync::Arc;
use std::{env, path::Path};

use confy::load_path;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::warn;

lazy_static! {
    pub static ref CONFIG_PATH: String = env::var("CONFIG_PATH").unwrap_or_else(|_| "./config.yaml".to_string());
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoresNodeConfig {
    pub public_key_hex: Option<String>,
    pub private_key_hex: Option<String>,
    pub identity_secret_seed_hex: Option<String>,
    pub network_name: Option<String>,
    pub region_name: Option<String>,
    pub bootstrap_node_ids: Option<Vec<String>>,
    pub region_ids: Option<Vec<String>>,
    pub hashed_admin_password: Option<String>,
}

impl ::std::default::Default for LoresNodeConfig {
    fn default() -> Self {
        Self {
            public_key_hex: None,
            private_key_hex: None,
            identity_secret_seed_hex: None,
            network_name: Some("lores".to_string()),
            region_name: None,
            region_ids: None,
            bootstrap_node_ids: None,
            hashed_admin_password: None,
        }
    }
}

impl LoresNodeConfig {
    pub fn load() -> Self {
        load_path(Path::new(&*CONFIG_PATH)).unwrap_or_else(|e| {
            warn!("Failed to load config: {}", e);
            LoresNodeConfig::default()
        })
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        confy::store_path(Path::new(&*CONFIG_PATH), self).map_err(|e| {
            warn!("Failed to save config: {}", e);
            anyhow::anyhow!("Failed to save config: {}", e)
        })
    }
}

#[derive(Debug, Clone)]
pub struct LoresNodeConfigState {
    config: Arc<Mutex<LoresNodeConfig>>,
}

impl LoresNodeConfigState {
    pub fn new(config: &LoresNodeConfig) -> Self {
        Self {
            config: Arc::new(Mutex::new(config.clone())),
        }
    }

    pub async fn get(&self) -> LoresNodeConfig {
        self.config.lock().await.clone()
    }

    pub async fn update(&self, callback: impl FnOnce(LoresNodeConfig) -> LoresNodeConfig) -> Result<(), anyhow::Error> {
        let mut locked_config = self.config.lock().await;
        let changed_config = callback(locked_config.clone());
        let save_result = changed_config.save();

        match save_result {
            Ok(_) => {
                *locked_config = changed_config;
                Ok(())
            }
            Err(e) => {
                warn!("Failed to save config: {}", e);
                Err(e)
            }
        }
    }
}
