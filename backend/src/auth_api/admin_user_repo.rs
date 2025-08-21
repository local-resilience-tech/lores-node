use std::collections::HashSet;

use npwg::{generate_password_with_config, PasswordGeneratorConfig, PasswordGeneratorError};
use password_auth::generate_hash;

use crate::config::config_state::LoresNodeConfigState;

pub enum GeneratePasswordError {
    PasswordAlreadySet,
    ServerError,
}

pub struct AdminUserRepo {}

impl AdminUserRepo {
    pub fn new() -> Self {
        AdminUserRepo {}
    }

    pub async fn generate_and_save_admin_password(
        &self,
        config_state: &LoresNodeConfigState,
    ) -> Result<String, GeneratePasswordError> {
        if self.password_already_set(config_state).await {
            return Err(GeneratePasswordError::PasswordAlreadySet);
        }

        let password = self.generate_admin_password().await.map_err(|e| {
            eprintln!("Error generating password: {}", e);
            GeneratePasswordError::ServerError
        })?;

        // Hash the password and store in config
        let hashed_password = generate_hash(&password);

        config_state
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

    async fn generate_admin_password(&self) -> Result<String, PasswordGeneratorError> {
        let mut pw_config = PasswordGeneratorConfig::new();
        pw_config.length = 20;
        pw_config.excluded_chars = HashSet::from([':', ';', '"']);

        generate_password_with_config(&pw_config).await
    }

    async fn password_already_set(&self, config_state: &LoresNodeConfigState) -> bool {
        let config = config_state.get().await;
        config.hashed_admin_password.is_some()
    }
}
