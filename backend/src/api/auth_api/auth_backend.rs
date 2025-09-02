use std::collections::HashSet;

use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, AuthzBackend, UserId};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};

use sqlx::SqlitePool;
use tokio::task;
use utoipa::ToSchema;

use crate::{
    config::config_state::LoresNodeConfigState,
    data::node_data::node_stewards::{NodeStewardIdentifier, NodeStewardsRepo},
};

use super::admin_user_repo::AdminUserRepo;

const ADMIN_USER_ID: &str = "admin";

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    password_hash: String,
}

impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.id.clone()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password_hash.as_bytes() // We use the password hash as the auth
                                      // hash--what this means
                                      // is when the user changes their password the
                                      // auth session becomes invalid.
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct AdminCredentials {
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct NodeStewardCredentials {
    pub id: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub enum Credentials {
    Admin(AdminCredentials),
    NodeSteward(NodeStewardCredentials),
}

#[derive(Debug, Clone)]
pub struct AppAuthBackend {
    config_state: LoresNodeConfigState,
    node_data_pool: SqlitePool,
}

impl AppAuthBackend {
    pub fn new(config_state: &LoresNodeConfigState, node_data_pool: &SqlitePool) -> Self {
        Self {
            config_state: config_state.clone(),
            node_data_pool: node_data_pool.clone(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials provided")]
    InvalidCredentials,
    #[error("No password set for user")]
    NoPasswordSet,
    #[error("Account is disabled")]
    AccountDisabled,
    #[error("Internal server error occurred")]
    ServerError,
}

#[async_trait]
impl AuthnBackend for AppAuthBackend {
    type User = User;
    type Credentials = Credentials;
    type Error = AuthError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, AuthError> {
        match creds {
            Self::Credentials::Admin(admin_creds) => {
                self.authenticate_admin_user(&admin_creds).await
            }
            Self::Credentials::NodeSteward(node_steward_creds) => {
                self.authenticate_steward_user(&node_steward_creds).await
            }
        }
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        if user_id.to_string() == ADMIN_USER_ID.to_string() {
            let user = Self::User {
                id: ADMIN_USER_ID.to_string(),
                password_hash: self.expect_hashed_password().await?,
            };

            return Ok(Some(user));
        } else {
            self.get_steward_user(user_id).await
        }
    }
}

impl AppAuthBackend {
    async fn authenticate_admin_user(
        &self,
        creds: &AdminCredentials,
    ) -> Result<Option<User>, AuthError> {
        println!("Authenticating admin user");

        let hashed_password = self.expect_hashed_password().await?;

        self.verify_password(creds.password.clone(), hashed_password.clone())
            .await?;

        Ok(Some(User {
            id: ADMIN_USER_ID.into(),
            password_hash: hashed_password,
        }))
    }

    async fn authenticate_steward_user(
        &self,
        creds: &NodeStewardCredentials,
    ) -> Result<Option<User>, AuthError> {
        println!("Authenticating admin user");

        let repo = NodeStewardsRepo::init();

        // Fetch node steward by ID
        let id = NodeStewardIdentifier {
            id: creds.id.clone(),
        };
        let steward = match repo.find(&self.node_data_pool, &id).await {
            Ok(Some(steward)) => steward,
            Ok(None) => {
                eprintln!("AUTH FAILED: Node steward not found");
                return Err(AuthError::UserNotFound);
            }
            Err(e) => {
                eprintln!("AUTH FAILED: Failed to find node steward: {:?}", e);
                return Err(AuthError::ServerError);
            }
        };

        // Check if the steward has a password set
        let hashed_password = match steward.hashed_password.clone() {
            Some(hash) => hash,
            None => return Err(AuthError::NoPasswordSet),
        };

        // Check if disabled
        if !steward.enabled {
            eprintln!("AUTH FAILED: The specified node steward account is disabled");
            return Err(AuthError::AccountDisabled);
        }

        // Check if the password matches the credentials
        self.verify_password(creds.password.clone(), hashed_password.clone())
            .await?;

        Ok(Some(User {
            id: steward.id.clone(),
            password_hash: hashed_password,
        }))
    }

    async fn get_steward_user(&self, user_id: &UserId<Self>) -> Result<Option<User>, AuthError> {
        let repo = NodeStewardsRepo::init();

        // Fetch node steward by ID
        let id = NodeStewardIdentifier {
            id: user_id.clone(),
        };
        let steward = match repo.find(&self.node_data_pool, &id).await {
            Ok(Some(steward)) => steward,
            Ok(None) => {
                eprintln!("RETRIEVE USER FAILED: Node steward not found");
                return Err(AuthError::UserNotFound);
            }
            Err(e) => {
                eprintln!("RETRIEVE USER FAILED: Failed to find node steward: {:?}", e);
                return Err(AuthError::ServerError);
            }
        };

        if !steward.enabled {
            eprintln!("RETRIEVE USER FAILED: The specified node steward account is disabled");
            return Err(AuthError::AccountDisabled);
        }

        match steward.hashed_password.clone() {
            Some(password_hash) => Ok(Some(User {
                id: steward.id.clone(),
                password_hash,
            })),
            None => return Err(AuthError::NoPasswordSet),
        }
    }

    async fn expect_hashed_password(&self) -> Result<String, AuthError> {
        let repo = AdminUserRepo::new(&self.config_state);
        let hashed_password = repo.get_hashed_password().await;

        match hashed_password {
            Some(hash) => Ok(hash),
            None => Err(AuthError::NoPasswordSet),
        }
    }

    async fn verify_password(&self, password: String, hashed: String) -> Result<(), AuthError> {
        // Verifying the password is blocking and potentially slow, so we'll do so via
        // `spawn_blocking`.
        let verification_result = task::spawn_blocking(move || verify_password(&password, &hashed))
            .await
            .map_err(|_| {
                eprintln!("Spawn blocking error when trying to verify password");
                AuthError::ServerError
            })?;

        verification_result.map_err(|_| {
            eprintln!("AUTH FAILED: Failed to verify password");
            AuthError::InvalidCredentials
        })?;

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Permission {
    pub name: String,
}

impl From<&str> for Permission {
    fn from(name: &str) -> Self {
        Permission {
            name: name.to_string(),
        }
    }
}

#[async_trait]
impl AuthzBackend for AppAuthBackend {
    type Permission = Permission;

    async fn get_group_permissions(
        &self,
        user: &Self::User,
    ) -> Result<HashSet<Self::Permission>, Self::Error> {
        let mut perms = HashSet::new();

        if user.id == ADMIN_USER_ID.to_string() {
            perms.insert(Permission::from("admin"));
        } else {
            perms.insert(Permission::from("steward"));
        }

        return Ok(perms);
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<AppAuthBackend>;
