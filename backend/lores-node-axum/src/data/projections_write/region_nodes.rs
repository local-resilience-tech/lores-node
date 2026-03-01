use sqlx::SqlitePool;

use crate::{data::entities::RegionNodeStatus, panda_comms::lores_events::RegionNodeUpdatedDataV1};

use super::nodes::NodesWriteRepo;

pub struct RegionNodesWriteRepo {}

impl RegionNodesWriteRepo {
    pub fn init() -> Self {
        RegionNodesWriteRepo {}
    }

    pub async fn upsert_join_status_and_details(
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

    pub async fn upsert_join_status(
        &self,
        pool: &SqlitePool,
        node_id: &str,
        region_id: &str,
        status: RegionNodeStatus,
    ) -> Result<(), sqlx::Error> {
        let node_repo = NodesWriteRepo::init();
        node_repo.upsert_id(pool, node_id).await?;

        sqlx::query!(
            "INSERT INTO region_nodes (node_id, region_id, status)
            VALUES (?, ?, ?)
            ON CONFLICT(node_id, region_id) DO UPDATE SET status = excluded.status",
            node_id,
            region_id,
            status,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn upsert_details(
        &self,
        pool: &SqlitePool,
        data: &RegionNodeUpdatedDataV1,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO region_nodes (
                node_id, region_id, name , public_ipv4, domain_on_local_network, domain_on_internet
            )
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(node_id, region_id) DO UPDATE SET
                name = excluded.name,
                public_ipv4 = excluded.public_ipv4,
                domain_on_local_network = excluded.domain_on_local_network,
                domain_on_internet = excluded.domain_on_internet",
            data.node_id,
            data.region_id,
            data.name,
            data.public_ipv4,
            data.domain_on_local_network,
            data.domain_on_internet
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
