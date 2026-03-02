use sqlx::{Sqlite, SqlitePool};

use crate::{
    api::public_api::client_events::ClientEvent,
    data::{
        entities::AppInstallation,
        projections_read::apps::AppsReadRepo,
        projections_write::{
            app_installations::AppInstallationsWriteRepo, region_nodes::RegionNodesWriteRepo,
        },
    },
    event_handlers::{
        utilities::{
            handle_db_write_error, header_has_region, region_utils::region_already_projected,
            HandlerResult,
        },
        EventHandler,
    },
    panda_comms::lores_events::{AppRegisteredDataV1, LoResEventHeader},
};

pub struct AppRegisteredHandler {
    payload: AppRegisteredDataV1,
}

impl AppRegisteredHandler {
    pub fn new(payload: &AppRegisteredDataV1) -> Self {
        Self {
            payload: payload.clone(),
        }
    }

    async fn write_projections(
        &self,
        header: LoResEventHeader,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        let node_write_repo = RegionNodesWriteRepo::init();
        let installations_write_repo = AppInstallationsWriteRepo::init();

        let region_id = header.region_id.clone().unwrap();

        let region_node = node_write_repo
            .find_or_create_by_keys(pool, &header.author_node_id, &region_id.to_hex())
            .await?;

        let installation = AppInstallation {
            app_name: self.payload.name.clone(),
            region_node_id: region_node.id.clone(),
            version: self.payload.version.clone(),
        };
        installations_write_repo.upsert(pool, installation).await?;

        Ok(())
    }

    async fn read_region_app_updated_event(
        &self,
        pool: &SqlitePool,
        app_name: String,
    ) -> Vec<ClientEvent> {
        let app_details = AppsReadRepo::init()
            .find_with_installations(pool, app_name)
            .await;

        match app_details {
            Ok(Some(details)) => vec![ClientEvent::RegionAppUpdated(details)],
            Ok(None) => {
                println!("Node not found for announcement.");
                vec![]
            }
            Err(e) => {
                eprintln!("Error reading node details: {}", e);
                vec![]
            }
        }
    }
}

impl EventHandler for AppRegisteredHandler {
    async fn handle(&self, header: LoResEventHeader, pool: &sqlx::Pool<Sqlite>) -> HandlerResult {
        let app_name = self.payload.name.clone();
        let result = self.write_projections(header, pool).await;

        match result {
            Ok(()) => HandlerResult {
                client_events: self.read_region_app_updated_event(pool, app_name).await,
            },
            Err(e) => handle_db_write_error(e),
        }
    }

    async fn validate(&self, header: &LoResEventHeader, pool: &SqlitePool) -> Result<(), ()> {
        header_has_region(header)?;
        region_already_projected(header, pool).await?;
        Ok(())
    }
}
