use sqlx::SqlitePool;

use super::super::entities::Node;

pub struct NodesWriteRepo {}

impl NodesWriteRepo {
    pub fn init() -> Self {
        NodesWriteRepo {}
    }

    pub async fn upsert_name(
        &self,
        pool: &SqlitePool,
        id: &String,
        name: &String,
    ) -> Result<(), sqlx::Error> {
        let _node = sqlx::query!(
            "INSERT INTO nodes (id, name) VALUES (?, ?) ON CONFLICT(id) DO UPDATE SET name = excluded.name",
            id,
            name
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn upsert(&self, pool: &SqlitePool, node: &Node) -> Result<(), sqlx::Error> {
        let _node = sqlx::query!(
            "INSERT INTO nodes (id, name, public_ipv4, domain_on_local_network, domain_on_internet) VALUES (?, ?, ?, ?, ?) ON CONFLICT(id) DO UPDATE SET name = excluded.name, public_ipv4 = excluded.public_ipv4, domain_on_local_network = excluded.domain_on_local_network, domain_on_internet = excluded.domain_on_internet",
            node.id,
            node.name,
            node.public_ipv4,
            node.domain_on_local_network,
            node.domain_on_internet
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update(&self, pool: &SqlitePool, node: &Node) -> Result<(), sqlx::Error> {
        let _node = sqlx::query!(
            "UPDATE nodes SET name = ?, public_ipv4 = ?, domain_on_local_network = ?, domain_on_internet = ? WHERE id = ?",
            node.name,
            node.public_ipv4,
            node.domain_on_local_network,
            node.domain_on_internet,
            node.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
