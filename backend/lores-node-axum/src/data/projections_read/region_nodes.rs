use sqlx::SqlitePool;

use super::super::entities::{RegionNode, RegionNodeDetails, RegionNodeStatus};

pub struct RegionNodesReadRepo {}

impl RegionNodesReadRepo {
    pub fn init() -> Self {
        RegionNodesReadRepo {}
    }

    pub async fn find_by_keys(
        &self,
        pool: &SqlitePool,
        node_id: String,
        region_id: String,
    ) -> Result<Option<RegionNode>, sqlx::Error> {
        let node = sqlx::query_as!(
            RegionNode,
            "
            SELECT
                id, node_id, region_id, status as \"status: RegionNodeStatus\", name, public_ipv4, domain_on_local_network, domain_on_internet
            FROM region_nodes
            WHERE region_nodes.node_id = ? AND region_nodes.region_id = ?
            LIMIT 1
            ",
            node_id,
            region_id
        )
        .fetch_optional(pool)
        .await?;

        return Ok(node);
    }

    pub async fn find_detailed(
        &self,
        pool: &SqlitePool,
        node_id: String,
    ) -> Result<Option<RegionNodeDetails>, sqlx::Error> {
        let node = sqlx::query_as!(
            RegionNodeDetails,
            "
            SELECT
                id, region_nodes.node_id as node_id, region_id, status as \"status: RegionNodeStatus\", name, public_ipv4, domain_on_local_network, domain_on_internet, s.text as status_text, s.state as state
            FROM region_nodes
            LEFT JOIN current_node_statuses AS s ON region_nodes.id = s.region_node_id
            WHERE region_nodes.node_id = ?
            LIMIT 1
            ",
            node_id
        )
        .fetch_optional(pool)
        .await?;

        return Ok(node);
    }

    pub async fn all(&self, pool: &SqlitePool) -> Result<Vec<RegionNodeDetails>, sqlx::Error> {
        let nodes = sqlx::query_as!(
            RegionNodeDetails,
            "
            SELECT
                id, region_nodes.node_id as node_id, region_id, status as \"status: RegionNodeStatus\", name, public_ipv4, domain_on_local_network, domain_on_internet, s.text as status_text, s.state as state
            FROM region_nodes
            LEFT JOIN current_node_statuses AS s ON region_nodes.id = s.region_node_id"
        )
        .fetch_all(pool)
        .await?;

        Ok(nodes)
    }
}
