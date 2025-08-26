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
}

pub struct NodeStewardsRepo {}

impl NodeStewardsRepo {
    pub fn init() -> Self {
        NodeStewardsRepo {}
    }

    pub async fn all(&self, pool: &SqlitePool) -> Result<Vec<NodeStewardRow>, sqlx::Error> {
        let nodes = sqlx::query_as::<Sqlite, NodeStewardRow>(
            "
            SELECT id, name, hashed_password, password_reset_token, password_reset_token_expires_at, enabled
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
            INSERT INTO node_stewards (id, name, hashed_password, password_reset_token, password_reset_token_expires_at, enabled)
            VALUES (?, ?, ?, ?, ?, ?)
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
