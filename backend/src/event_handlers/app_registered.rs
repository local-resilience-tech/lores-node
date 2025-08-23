use sqlx::{Sqlite, SqlitePool};

use crate::{
    api::public_api::client_events::ClientEvent,
    event_handlers::handler_utilities::{handle_db_write_error, HandlerResult},
    panda_comms::lores_events::{AppRegisteredDataV1, LoResEventHeader},
    projections::{
        entities::{AppInstallation, RegionApp},
        projections_read::apps::AppsReadRepo,
        projections_write::{app_installations::AppInstallationsWriteRepo, apps::AppsWriteRepo},
    },
};

pub struct AppRegisteredHandler {}

impl AppRegisteredHandler {
    pub async fn handle(
        header: LoResEventHeader,
        payload: AppRegisteredDataV1,
        pool: &sqlx::Pool<Sqlite>,
    ) -> HandlerResult {
        println!("App registered: {:?}", payload);

        let app_name = payload.name.clone();
        let result = Self::write_projections(header, payload, pool).await;

        match result {
            Ok(()) => HandlerResult {
                client_events: Self::read_region_updated_event(pool, app_name).await,
            },
            Err(e) => handle_db_write_error(e),
        }
    }

    async fn write_projections(
        header: LoResEventHeader,
        payload: AppRegisteredDataV1,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        let repo = AppsWriteRepo::init();
        let app = RegionApp {
            name: payload.name.clone(),
        };
        repo.upsert(pool, app).await?;

        let repo = AppInstallationsWriteRepo::init();
        let installation = AppInstallation {
            app_name: payload.name.clone(),
            node_id: header.author_node_id.clone(),
            version: payload.version.clone(),
        };
        repo.upsert(pool, installation).await?;

        Ok(())
    }

    async fn read_region_updated_event(pool: &SqlitePool, app_name: String) -> Vec<ClientEvent> {
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
