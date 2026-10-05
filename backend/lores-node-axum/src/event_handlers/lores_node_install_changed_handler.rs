use sqlx::SqlitePool;

use crate::{
    data::projections_write::lores_node_installations::LoresNodeInstallationsWriteRepo,
    event_handlers::utilities::{EventHandler, HandlerResult, handle_db_write_error, header_has_region, read_node_updated_event},
    panda_comms::lores_events::{LoResEventHeader, LoresNodeInstallChangedDataV1},
};

pub struct LoresNodeInstallChangedHandler {
    payload: LoresNodeInstallChangedDataV1,
}

impl LoresNodeInstallChangedHandler {
    pub fn new(payload: &LoresNodeInstallChangedDataV1) -> Self {
        Self { payload: payload.clone() }
    }

    async fn write_projections(&self, header: &LoResEventHeader, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let lores_node_installations_write_repo = LoresNodeInstallationsWriteRepo::init();

        lores_node_installations_write_repo
            .upsert(pool, &header.author_node_id, &self.payload.lores_version)
            .await?;

        Ok(())
    }
}

impl EventHandler for LoresNodeInstallChangedHandler {
    async fn handle(&self, header: LoResEventHeader, pool: &SqlitePool) -> HandlerResult {
        let region_id = match &header.region_id {
            Some(id) => id,
            None => {
                tracing::warn!("Region ID is missing");
                return HandlerResult::default();
            }
        };

        let node_id = header.author_node_id.clone();
        let result = self.write_projections(&header, pool).await;

        match result {
            Ok(()) => HandlerResult {
                client_events: read_node_updated_event(pool, node_id, region_id.to_hex()).await,
            },
            Err(e) => handle_db_write_error(e),
        }
    }

    async fn validate(&self, header: &LoResEventHeader, _pool: &SqlitePool) -> Result<(), ()> {
        header_has_region(header)
    }
}

#[cfg(test)]
mod tests {
    use lores_p2panda::RegionId;
    use p2panda_core::Hash;

    use crate::{
        api::public_api::client_events::ClientEvent::RegionNodeUpdated,
        data::{
            entities::Region,
            projections_write::{nodes::NodesWriteRepo, region_nodes::RegionNodesWriteRepo, regions::RegionsWriteRepo},
        },
        panda_comms::lores_events::RegionNodeUpdatedDataV1,
    };

    use super::*;

    #[sqlx::test(migrations = "../migrations_projectiondb")]
    async fn returns_client_events(pool: SqlitePool) -> () {
        let region_id = RegionId::from_hex("b64cce8cfb94be72549a71b47d0dd614e27baf23d7372bea79730f62e3c15bb4").unwrap();
        let node_id = "test_node_id".to_string();
        let lores_version = "0.23.2".to_string();

        let node_write_repo = NodesWriteRepo::init();
        node_write_repo.upsert_id(&pool, &node_id).await.unwrap();

        let mut test_region = Region::default();
        test_region.id = region_id.to_hex();

        let region_write_repo = RegionsWriteRepo::init();
        region_write_repo.upsert(&pool, &test_region).await.unwrap();

        let region_nodes_write_repo = RegionNodesWriteRepo::init();
        region_nodes_write_repo
            .upsert_details(&pool, &region_id.to_hex(), &node_id, &RegionNodeUpdatedDataV1::default())
            .await
            .unwrap();

        let install_changed_data = LoresNodeInstallChangedDataV1 {
            node_id: node_id.clone(),
            lores_version: lores_version.clone(),
        };

        let event_header = LoResEventHeader {
            author_node_id: node_id.clone(),
            region_id: Some(region_id.clone()),
            timestamp: 1790919673,
            operation_id: Hash::digest(vec![0_u8]),
        };

        let result = LoresNodeInstallChangedHandler::new(&install_changed_data)
            .handle(event_header, &pool)
            .await
            .client_events;

        assert_eq!(1, result.len());

        if let RegionNodeUpdated(node) = &result.first().unwrap() {
            assert_eq!(node.node_id, node_id);
            assert_eq!(node.lores_version, Some(lores_version));
        };
    }
}
