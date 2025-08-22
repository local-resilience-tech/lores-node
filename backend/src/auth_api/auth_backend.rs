use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tokio::task;
use utoipa::ToSchema;

use crate::config::config_state::LoresNodeConfigState;

use super::{
    admin_user_repo::AdminUserRepo,
    auth_repo::{AuthRepo, AuthRepoError},
};

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    password: String,
}

// Here we've implemented `Debug` manually to avoid accidentally logging the
// password hash.
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("email", &self.email)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = i64;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes() // We use the password hash as the auth
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
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub enum Credentials {
    Admin(AdminCredentials),
    NodeSteward(NodeStewardCredentials),
}

#[derive(Debug, Clone)]
pub struct AppAuthBackend {
    db: SqlitePool,
    config_state: LoresNodeConfigState,
}

impl AppAuthBackend {
    pub fn new(db: &SqlitePool, config_state: &LoresNodeConfigState) -> Self {
        Self {
            db: db.clone(),
            config_state: config_state.clone(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid credentials provided")]
    InvalidCredentials,
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
        let repo = AuthRepo::new(&self.db);

        repo.user_for_id(*user_id)
            .await
            .map(|user| {
                user.map(|u| User {
                    id: u.id,
                    email: u.email.unwrap_or_default(),
                    password: u.hashed_password.unwrap_or_default(),
                })
            })
            .map_err(|e| {
                eprintln!("Error fetching user: {:?}", e);
                AuthError::UserNotFound
            })
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

        println!("Admin user authenticated successfully, still more TODO here");

        Ok(None)
    }

    async fn authenticate_steward_user(
        &self,
        creds: &NodeStewardCredentials,
    ) -> Result<Option<User>, AuthError> {
        println!("Authenticating node steward user: {:?}", creds);

        Ok(None)
    }

    async fn expect_hashed_password(&self) -> Result<String, AuthError> {
        let repo = AdminUserRepo::new(&self.config_state);
        let hashed_password = repo.get_hashed_password().await;

        match hashed_password {
            Some(hash) => Ok(hash),
            None => Err(AuthError::UserNotFound),
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

        verification_result.map_err(|_| AuthError::InvalidCredentials)?;

        Ok(())
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<AppAuthBackend>;
