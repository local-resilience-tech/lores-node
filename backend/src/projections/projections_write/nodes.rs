use sqlx::SqlitePool;

use super::super::entities::Node;

pub struct NodeUpdateRow {
    pub id: String,
    pub name: String,
    pub public_ipv4: Option<String>,
}

pub struct NodesWriteRepo {}

impl NodesWriteRepo {
    pub fn init() -> Self {
        NodesWriteRepo {}
    }

    pub async fn upsert(&self, pool: &SqlitePool, node: Node) -> Result<(), sqlx::Error> {
        let _node = sqlx::query!(
            "INSERT INTO nodes (id, name) VALUES (?, ?) ON CONFLICT(id) DO UPDATE SET name = ?",
            node.id,
            node.name,
            node.name
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update(&self, pool: &SqlitePool, node: NodeUpdateRow) -> Result<(), sqlx::Error> {
        let _node = sqlx::query!(
            "UPDATE nodes SET name = ?, public_ipv4 = ? WHERE id = ?",
            node.name,
            node.public_ipv4,
            node.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
