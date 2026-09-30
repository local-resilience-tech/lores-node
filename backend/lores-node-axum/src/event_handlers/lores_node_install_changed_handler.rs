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
            .await;

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

// to do add tests
