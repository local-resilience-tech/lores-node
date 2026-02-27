use sqlx::SqlitePool;

use crate::data::entities::{RegionNode, RegionNodeStatus};

use super::nodes::NodesWriteRepo;

pub struct RegionNodesWriteRepo {}

impl RegionNodesWriteRepo {
    pub fn init() -> Self {
        RegionNodesWriteRepo {}
    }

    pub async fn upsert_name(
        &self,
        pool: &SqlitePool,
        node_id: &str,
        region_id: &str,
        name: &str,
    ) -> Result<(), sqlx::Error> {
        let node_repo = NodesWriteRepo::init();
        node_repo.upsert_id(pool, node_id).await?;

        sqlx::query!(
            "INSERT INTO region_nodes (node_id, region_id, name)
            VALUES (?, ?, ?)
            ON CONFLICT(node_id, region_id) DO UPDATE SET name = excluded.name",
            node_id,
            region_id,
            name
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn upsert_join_status(
        &self,
        pool: &SqlitePool,
        node_id: &str,
        region_id: &str,
        status: RegionNodeStatus,
        about_your_node: Option<String>,
        about_your_stewards: Option<String>,
        agreed_node_steward_conduct_url: Option<String>,
    ) -> Result<(), sqlx::Error> {
        let node_repo = NodesWriteRepo::init();
        node_repo.upsert_id(pool, node_id).await?;

        sqlx::query!(
            "INSERT INTO region_nodes (node_id, region_id, status, about_your_node, about_your_stewards, agreed_node_steward_conduct_url)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(node_id, region_id) DO UPDATE SET status = excluded.status, about_your_node = excluded.about_your_node, about_your_stewards = excluded.about_your_stewards, agreed_node_steward_conduct_url = excluded.agreed_node_steward_conduct_url",
            node_id,
            region_id,
            status,
            about_your_node,
            about_your_stewards,
            agreed_node_steward_conduct_url
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn upsert(&self, pool: &SqlitePool, node: &RegionNode) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO region_nodes (node_id, region_id, status, name, public_ipv4, domain_on_local_network, domain_on_internet)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(node_id, region_id) DO UPDATE SET status = excluded.status, name = excluded.name, public_ipv4 = excluded.public_ipv4, domain_on_local_network = excluded.domain_on_local_network, domain_on_internet = excluded.domain_on_internet",
            node.node_id,
            node.region_id,
            node.status,
            node.name,
            node.public_ipv4,
            node.domain_on_local_network,
            node.domain_on_internet
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update(&self, pool: &SqlitePool, node: &RegionNode) -> Result<(), sqlx::Error> {
        let _node = sqlx::query!(
            "UPDATE region_nodes
            SET status = ?, name = ?, public_ipv4 = ?, domain_on_local_network = ?, domain_on_internet = ?
            WHERE node_id = ?",
            node.status,
            node.name,
            node.public_ipv4,
            node.domain_on_local_network,
            node.domain_on_internet,
            node.node_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
