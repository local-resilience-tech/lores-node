use super::entities::Node;
use crate::repos::helpers::NODE_CONFIG_ID;
use sqlx::SqlitePool;

pub struct ThisNodeRepo {}

impl ThisNodeRepo {
    pub fn init() -> Self {
        ThisNodeRepo {}
    }

    pub async fn find(&self, pool: &SqlitePool) -> Result<Option<Node>, sqlx::Error> {
        let node = sqlx::query_as!(
            Node,
            "
            SELECT nodes.id as id, nodes.name as name
            FROM nodes
            INNER JOIN node_configs ON node_configs.public_key_hex = nodes.id
            WHERE node_configs.id = ? LIMIT 1
            ",
            NODE_CONFIG_ID
        )
        .fetch_optional(pool)
        .await?;

        return Ok(node);
    }
}
