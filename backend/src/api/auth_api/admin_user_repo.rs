use std::collections::HashSet;

use npwg::{generate_password_with_config, PasswordGeneratorConfig, PasswordGeneratorError};
use password_auth::generate_hash;

use crate::config::config_state::LoresNodeConfigState;

pub enum GeneratePasswordError {
    PasswordAlreadySet,
    ServerError,
}

pub struct AdminUserRepo {
    config_state: LoresNodeConfigState,
}

impl AdminUserRepo {
    pub fn new(config_state: &LoresNodeConfigState) -> Self {
        AdminUserRepo {
            config_state: config_state.clone(),
        }
    }

    pub async fn generate_and_save_admin_password(&self) -> Result<String, GeneratePasswordError> {
        if self.password_already_set().await {
            return Err(GeneratePasswordError::PasswordAlreadySet);
        }

        let password = self.generate_admin_password().await.map_err(|e| {
            eprintln!("Error generating password: {}", e);
            GeneratePasswordError::ServerError
        })?;

        // Hash the password and store in config
        let hashed_password = generate_hash(&password);

        self.config_state
            .update(|config| {
                let mut result = config.clone();
                result.hashed_admin_password = Some(hashed_password);
                result
            })
            .await
            .map_err(|e| {
                eprintln!("Error storing admin password in config: {}", e);
                GeneratePasswordError::ServerError
            })?;

        return Ok(password);
    }

    pub async fn get_hashed_password(&self) -> Option<String> {
        let config = self.config_state.get().await;
        config.hashed_admin_password.clone()
    }

    async fn generate_admin_password(&self) -> Result<String, PasswordGeneratorError> {
        let mut pw_config = PasswordGeneratorConfig::new();
        pw_config.length = 20;
        pw_config.excluded_chars = HashSet::from([':', ';', '"']);

        generate_password_with_config(&pw_config).await
    }

    async fn password_already_set(&self) -> bool {
        self.get_hashed_password().await.is_some()
    }
}
