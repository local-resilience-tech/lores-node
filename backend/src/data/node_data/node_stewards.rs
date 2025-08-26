use chrono::NaiveDateTime;
use sqlx::{Sqlite, SqlitePool};

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

impl NodeStewardRow {
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
            VALUES (?, ?, ?, ?, ?, ?, NOW(), NOW())
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
}
