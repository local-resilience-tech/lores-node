use sqlx::SqlitePool;

use crate::projections::entities::Node;

use super::super::entities::NodeDetails;

pub struct NodesReadRepo {}

impl NodesReadRepo {
    pub fn init() -> Self {
        NodesReadRepo {}
    }

    pub async fn find(
        &self,
        pool: &SqlitePool,
        node_id: String,
    ) -> Result<Option<Node>, sqlx::Error> {
        let node = sqlx::query_as!(
            Node,
            "
            SELECT nodes.id as id, nodes.name as name
            FROM nodes
            WHERE nodes.id = ?
            LIMIT 1
            ",
            node_id
        )
        .fetch_optional(pool)
        .await?;

        return Ok(node);
    }

    pub async fn find_detailed(
        &self,
        pool: &SqlitePool,
        node_id: String,
    ) -> Result<Option<NodeDetails>, sqlx::Error> {
        let node = sqlx::query_as!(
            NodeDetails,
            "
            SELECT id, name, public_ipv4, s.text as status_text, s.state as state
            FROM nodes
            LEFT JOIN current_node_statuses AS s ON nodes.id = s.node_id
            WHERE nodes.id = ?
            LIMIT 1
            ",
            node_id
        )
        .fetch_optional(pool)
        .await?;

        return Ok(node);
    }

    pub async fn all(&self, pool: &SqlitePool) -> Result<Vec<NodeDetails>, sqlx::Error> {
        let nodes = sqlx::query_as!(
            NodeDetails,
            "
            SELECT id, name, public_ipv4, s.text as status_text, s.state as state
            FROM nodes
            LEFT JOIN current_node_statuses AS s ON nodes.id = s.node_id"
        )
        .fetch_all(pool)
        .await?;

        Ok(nodes)
    }
}
