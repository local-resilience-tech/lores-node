use sqlx::SqlitePool;

use super::entities::{Node, NodeDetails};

pub struct NodesRepo {}

pub struct NodeUpdateRow {
    pub id: String,
    pub name: String,
    pub public_ipv4: Option<String>,
}

impl NodesRepo {
    pub fn init() -> Self {
        NodesRepo {}
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

    pub async fn all(&self, pool: &SqlitePool) -> Result<Vec<NodeDetails>, sqlx::Error> {
        let nodes = sqlx::query_as!(
            NodeDetails,
            "SELECT id, name, public_ipv4, s.text as status_text, s.state as state FROM nodes LEFT JOIN current_node_statuses AS s ON nodes.id = s.node_id"
        )
        .fetch_all(pool)
        .await?;

        Ok(nodes)
    }
}
