use sqlx::{Sqlite, SqlitePool};

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
pub struct NodeStewardRow {
    pub id: String,
    pub name: String,
    pub hashed_password: Option<String>,
    pub password_reset_token: Option<String>,
    pub active: bool,
}

pub struct NodeStewardsRepo {}

impl NodeStewardsRepo {
    pub fn init() -> Self {
        NodeStewardsRepo {}
    }

    pub async fn all(&self, pool: &SqlitePool) -> Result<Vec<NodeStewardRow>, sqlx::Error> {
        let nodes = sqlx::query_as::<Sqlite, NodeStewardRow>(
            "
            SELECT id, name, hashed_password, password_reset_token, active
            FROM node_stewards
            ",
        )
        .fetch_all(pool)
        .await?;

        Ok(nodes)
    }
}
