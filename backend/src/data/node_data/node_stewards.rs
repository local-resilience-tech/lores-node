use chrono::NaiveDateTime;
use pwgen2::pwgen::{generate_password, PasswordConfig};
use serde::Serialize;
use short_uuid::ShortUuid;
use sqlx::{Sqlite, SqlitePool};
use utoipa::ToSchema;

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
pub struct NodeStewardRow {
    pub id: String,
    pub name: String,
    pub hashed_password: Option<String>,
    pub password_reset_token: Option<String>,
    pub password_reset_token_expires_at: Option<NaiveDateTime>,
    pub enabled: bool,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, ToSchema)]
pub struct NodeStewardIdentifier {
    pub id: String,
}

impl NodeStewardRow {
    pub fn new(name: String) -> Self {
        NodeStewardRow {
            id: new_node_steward_id(),
            name,
            hashed_password: None,
            password_reset_token: None,
            password_reset_token_expires_at: None,
            enabled: true,
            created_at: None,
        }
    }

    pub fn token_expired(&self) -> bool {
        if self.hashed_password.is_some() {
            return false;
        }

        if self.password_reset_token.is_none() {
            return true;
        }

        match self.password_reset_token_expires_at {
            Some(expiry) => {
                let now = chrono::Utc::now().naive_utc();
                now > expiry
            }
            None => true, // No expiry means no valid token
        }
    }

    pub fn set_password_reset_token(&mut self) {
        self.password_reset_token = Some(new_password_reset_token());
        self.password_reset_token_expires_at = Some(new_reset_token_expiry());
    }

    pub fn token_equals(&self, token: &str) -> bool {
        if let Some(stored_token) = &self.password_reset_token {
            stored_token == token && !stored_token.is_empty()
        } else {
            false
        }
    }
}

pub struct NodeStewardsRepo {}

impl NodeStewardsRepo {
    pub fn init() -> Self {
        NodeStewardsRepo {}
    }

    pub async fn all(&self, pool: &SqlitePool) -> Result<Vec<NodeStewardRow>, sqlx::Error> {
        let nodes = sqlx::query_as::<Sqlite, NodeStewardRow>(
            "
            SELECT id, name, hashed_password, password_reset_token, password_reset_token_expires_at, enabled, created_at
            FROM node_stewards
            ",
        )
        .fetch_all(pool)
        .await?;

        Ok(nodes)
    }

    pub async fn create(&self, pool: &SqlitePool, row: &NodeStewardRow) -> Result<(), sqlx::Error> {
        sqlx::query::<Sqlite>(
            "
            INSERT INTO node_stewards (id, name, hashed_password, password_reset_token, password_reset_token_expires_at, enabled, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            ",
        )
        .bind(&row.id)
        .bind(&row.name)
        .bind(&row.hashed_password)
        .bind(&row.password_reset_token)
        .bind(&row.password_reset_token_expires_at)
        .bind(row.enabled)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find(
        &self,
        pool: &SqlitePool,
        identifier: &NodeStewardIdentifier,
    ) -> Result<Option<NodeStewardRow>, sqlx::Error> {
        let row = sqlx::query_as::<Sqlite, NodeStewardRow>(
            "
            SELECT id, name, hashed_password, password_reset_token, password_reset_token_expires_at, enabled, created_at
            FROM node_stewards
            WHERE id = ?
            ",
        )
        .bind(identifier.id.clone())
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }

    pub async fn update_password_reset_token(
        &self,
        pool: &SqlitePool,
        row: &NodeStewardRow,
    ) -> Result<(), sqlx::Error> {
        sqlx::query::<Sqlite>(
            "
            UPDATE node_stewards
            SET password_reset_token = ?, password_reset_token_expires_at = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            ",
        )
        .bind(&row.password_reset_token)
        .bind(&row.password_reset_token_expires_at)
        .bind(&row.id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update_password_and_clear_token(
        &self,
        pool: &SqlitePool,
        identifier: &NodeStewardIdentifier,
        hashed_password: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query::<Sqlite>(
            "
            UPDATE node_stewards
            SET hashed_password = ?, password_reset_token = NULL, password_reset_token_expires_at = NULL, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            ",
        )
        .bind(hashed_password)
        .bind(&identifier.id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update_enabled(
        &self,
        pool: &SqlitePool,
        identifier: &NodeStewardIdentifier,
        enabled: bool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query::<Sqlite>(
            "
            UPDATE node_stewards
            SET enabled = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            ",
        )
        .bind(enabled)
        .bind(&identifier.id)
        .execute(pool)
        .await?;

        Ok(())
    }
}

fn new_node_steward_id() -> String {
    ShortUuid::generate().to_string()
}

fn new_password_reset_token() -> String {
    let pw_config = PasswordConfig::alphanumeric(8).unwrap();
    generate_password(&pw_config)
}

fn new_reset_token_expiry() -> NaiveDateTime {
    chrono::Utc::now().naive_utc() + chrono::Duration::hours(24)
}
