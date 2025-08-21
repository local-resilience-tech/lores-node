use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use password_auth::verify_password;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tokio::task;
use utoipa::ToSchema;

use super::auth_repo::{AuthRepo, AuthRepoError};

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
}

impl AppAuthBackend {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),

    #[error(transparent)]
    AuthRepo(#[from] AuthRepoError),
}

#[async_trait]
impl AuthnBackend for AppAuthBackend {
    type User = User;
    type Credentials = Credentials;
    type Error = Error;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(None)
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
            .map_err(|e| Error::AuthRepo(e))
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<AppAuthBackend>;
