use sqlx::SqlitePool;

use super::super::entities::Node;

pub struct ThisNodeReadRepo {}

impl ThisNodeReadRepo {
    pub fn init() -> Self {
        ThisNodeReadRepo {}
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
}
