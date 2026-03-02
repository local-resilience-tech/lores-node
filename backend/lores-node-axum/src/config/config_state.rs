use std::sync::Arc;
use tokio::sync::Mutex;

use super::config::LoresNodeConfig;

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

    pub async fn update(
        &self,
        callback: impl FnOnce(LoresNodeConfig) -> LoresNodeConfig,
    ) -> Result<(), anyhow::Error> {
        let mut locked_config = self.config.lock().await;
        let changed_config = callback(locked_config.clone());
        let save_result = changed_config.save();

        match save_result {
            Ok(_) => {
                *locked_config = changed_config;
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to save config: {}", e);
                Err(e)
            }
        }
    }
}
