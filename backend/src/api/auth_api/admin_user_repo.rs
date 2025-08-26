use password_auth::generate_hash;
use pwgen2::pwgen::{generate_password, PasswordConfig};

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

        let password = self.generate_admin_password();

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

    fn generate_admin_password(&self) -> String {
        let pw_config = PasswordConfig::new(20).unwrap();
        generate_password(&pw_config)
    }

    async fn password_already_set(&self) -> bool {
        self.get_hashed_password().await.is_some()
    }
}
