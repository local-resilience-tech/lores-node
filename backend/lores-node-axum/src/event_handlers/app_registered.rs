use sqlx::{Sqlite, SqlitePool};

use crate::{
    api::public_api::client_events::ClientEvent,
    data::projections_read::apps::AppsReadRepo,
    event_handlers::{
        utilities::{handle_db_write_error, HandlerResult},
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
        _header: LoResEventHeader,
        _pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        // let repo = AppsWriteRepo::init();
        // let app = RegionApp {
        //     name: payload.name.clone(),
        // };
        // repo.upsert(pool, app).await?;

        // let repo = AppInstallationsWriteRepo::init();
        // let installation = AppInstallation {
        //     app_name: payload.name.clone(),
        //     region_node_id: header.author_node_id.clone(),
        //     version: payload.version.clone(),
        // };
        // repo.upsert(pool, installation).await?;

        Ok(())
    }

    async fn read_region_updated_event(
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
                client_events: self.read_region_updated_event(pool, app_name).await,
            },
            Err(e) => handle_db_write_error(e),
        }
    }
}
