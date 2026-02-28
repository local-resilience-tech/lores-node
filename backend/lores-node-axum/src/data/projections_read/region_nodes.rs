use sqlx::SqlitePool;

use crate::data::entities::{Region, RegionWithNodes};

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

    pub async fn append_detail_nodes_to_list(
        &self,
        pool: &SqlitePool,
        regions: Vec<Region>,
    ) -> Result<Vec<RegionWithNodes>, sqlx::Error> {
        let mut result = Vec::new();
        for region in regions {
            let with_details = self.append_detailed_nodes(pool, &region).await?;
            result.push(with_details);
        }
        Ok(result)
    }

    pub async fn append_detailed_nodes(
        &self,
        pool: &SqlitePool,
        region: &Region,
    ) -> Result<RegionWithNodes, sqlx::Error> {
        let nodes = self.find_all_detailed(pool, &region.id).await?;

        let with_details = RegionWithNodes {
            region: region.clone(),
            nodes,
        };
        Ok(with_details)
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

    pub async fn find_all_detailed(
        &self,
        pool: &SqlitePool,
        region_id: &str,
    ) -> Result<Vec<RegionNodeDetails>, sqlx::Error> {
        let nodes = sqlx::query_as!(
            RegionNodeDetails,
            "
            SELECT
                id, region_nodes.node_id as node_id, region_id, status as \"status: RegionNodeStatus\", name, public_ipv4, domain_on_local_network, domain_on_internet, s.text as status_text, s.state as state
            FROM region_nodes
            LEFT JOIN current_node_statuses AS s ON region_nodes.id = s.region_node_id
            WHERE region_nodes.region_id = ?
            ",
            region_id
        )
        .fetch_all(pool)
        .await?;

        Ok(nodes)
    }
}
