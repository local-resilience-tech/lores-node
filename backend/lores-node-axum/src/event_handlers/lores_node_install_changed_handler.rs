use sqlx::SqlitePool;

use crate::{
    api::public_api::client_events::ClientEvent,
    data::{
        projections_read::lores_node_installations::LoresNodeInstallationsReadRepo,
        projections_write::lores_node_installations::LoresNodeInstallationsWriteRepo,
    },
    event_handlers::utilities::{EventHandler, HandlerResult, handle_db_write_error, header_has_region},
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

    async fn read_node_installation_event(&self, pool: &SqlitePool) -> Vec<ClientEvent> {
        let lores_node_installations_read_repo = LoresNodeInstallationsReadRepo::init();

        let node = lores_node_installations_read_repo
            .find_by_node_id(pool, &self.payload.node_id)
            .await;

        match node {
            Ok(Some(details)) => vec![ClientEvent::LoresNodeInstallationChanged(details)],
            Ok(None) => {
                tracing::info!("Node not found for {}", self.payload.node_id);
                vec![]
            }
            Err(e) => {
                tracing::error!("Error reading node details for {}: {}", self.payload.node_id, e);
                vec![]
            }
        }
    }
}

impl EventHandler for LoresNodeInstallChangedHandler {
    async fn handle(&self, header: LoResEventHeader, pool: &SqlitePool) -> HandlerResult {
        let result = self.write_projections(&header, pool).await;

        match result {
            Ok(()) => HandlerResult {
                client_events: self.read_node_installation_event(pool).await,
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

    use crate::api::public_api::client_events::ClientEvent::LoresNodeInstallationChanged;

    use super::*;

    #[sqlx::test(migrations = "../migrations_projectiondb")]
    async fn returns_client_events(pool: SqlitePool) -> () {
        let node_id = "test_node_id".to_string();
        let lores_version = "0.23.2".to_string();

        let install_changed_data = LoresNodeInstallChangedDataV1 {
            node_id: node_id.clone(),
            lores_version: lores_version.clone(),
        };

        let event_header = LoResEventHeader {
            author_node_id: node_id.clone(),
            region_id: Some(RegionId::from_hex("b64cce8cfb94be72549a71b47d0dd614e27baf23d7372bea79730f62e3c15bb4").unwrap()),
            timestamp: 1790919673,
            operation_id: Hash::digest(vec![0_u8]),
        };

        let result = LoresNodeInstallChangedHandler::new(&install_changed_data)
            .handle(event_header, &pool)
            .await
            .client_events;

        assert_eq!(1, result.len());

        if let LoresNodeInstallationChanged(node) = &result.first().unwrap() {
            assert_eq!(node.node_id, node_id);
            assert_eq!(node.lores_version, Some(lores_version));
        };
    }
}
