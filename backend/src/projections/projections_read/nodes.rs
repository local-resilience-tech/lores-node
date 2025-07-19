use sqlx::SqlitePool;

use super::super::entities::NodeDetails;

pub struct NodesReadRepo {}

impl NodesReadRepo {
    pub fn init() -> Self {
        NodesReadRepo {}
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
