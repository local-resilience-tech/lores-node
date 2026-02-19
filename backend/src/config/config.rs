use confy::load_path;
use serde::{Deserialize, Serialize};

use std::{env, path::Path};

lazy_static! {
    pub static ref CONFIG_PATH: String =
        env::var("CONFIG_PATH").unwrap_or_else(|_| "./config.yaml".to_string());
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoresNodeConfig {
    pub public_key_hex: Option<String>,
    pub private_key_hex: Option<String>,
    pub bootstrap_node_id: Option<String>,
    pub hashed_admin_password: Option<String>,
}

impl ::std::default::Default for LoresNodeConfig {
    fn default() -> Self {
        Self {
            public_key_hex: None,
            private_key_hex: None,
            bootstrap_node_id: None,
            hashed_admin_password: None,
        }
    }
}

impl LoresNodeConfig {
    pub fn load() -> Self {
        load_path(Path::new(&*CONFIG_PATH)).unwrap_or_else(|e| {
            eprintln!("Failed to load config: {}", e);
            LoresNodeConfig::default()
        })
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        confy::store_path(Path::new(&*CONFIG_PATH), self).map_err(|e| {
            eprintln!("Failed to save config: {}", e);
            anyhow::anyhow!("Failed to save config: {}", e)
        })
    }
}
