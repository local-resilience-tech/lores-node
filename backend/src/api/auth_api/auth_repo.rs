use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthUser {
    pub id: i64,
    pub email: Option<String>,
    pub hashed_password: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthRepoError {
    #[error("User not found")]
    UserNotFound,
    #[error("Database error")]
    DatabaseError,
}

pub struct AuthRepo<'a> {
    pool: &'a sqlx::SqlitePool,
}

impl<'a> AuthRepo<'a> {
    pub fn new(pool: &'a sqlx::SqlitePool) -> Self {
        AuthRepo { pool }
    }

    // pub async fn user_for_email(&self, email: String) -> Result<Option<AuthUser>, AuthRepoError> {
    //     Err(AuthRepoError::UserNotFound) // Placeholder for actual implementation
    // }

    pub async fn user_for_id(&self, user_id: i64) -> Result<Option<AuthUser>, AuthRepoError> {
        Err(AuthRepoError::UserNotFound) // Placeholder for actual implementation
    }

    // pub async fn set_password_reset_token(
    //     &self,
    //     user_id: i64,
    //     token: String,
    // ) -> Result<(), AuthRepoError> {
    //     Err(AuthRepoError::UserNotFound) // Placeholder for actual implementation
    // }

    // pub async fn set_password_if_token_valid(
    //     &self,
    //     token: String,
    //     new_hashed_password: String,
    //     hours_token_valid: u32,
    // ) -> Result<(), AuthRepoError> {
    //     Err(AuthRepoError::UserNotFound) // Placeholder for actual implementation
    // }
}

// fn log_and_return_db_error(error: sqlx::Error) -> AuthRepoError {
//     eprintln!("Database error: {}", error);
//     AuthRepoError::DatabaseError
// }
