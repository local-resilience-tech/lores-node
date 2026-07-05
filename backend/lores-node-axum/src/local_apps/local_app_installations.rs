use std::sync::Arc;

use lores_p2panda::RegionId;
use sqlx::SqlitePool;

use crate::data::{
    entities::{LocalApp, LocalAppInstallation},
    node_data::app_installations_repo::AppInstallationsRepo,
    projections_read::apps::AppsReadRepo,
};

pub async fn build_local_app_installations(
    pool: &SqlitePool,
    node_id_hex: &str,
    local_apps: Vec<LocalApp>,
) -> Result<Vec<LocalAppInstallation>, sqlx::Error> {
    let app_names: Vec<String> = local_apps.iter().map(|a| a.name.clone()).collect();

    let region_ids = AppsReadRepo::init()
        .find_region_ids_for_apps(pool, &app_names, node_id_hex)
        .await?;

    let installations = local_apps
        .into_iter()
        .map(|app| {
            let region_id = region_ids.get(&app.name).cloned();
            LocalAppInstallation { app, region_id }
        })
        .collect();

    Ok(installations)
}

/// Returns a callback suitable for passing to `PandaService::new` that records
/// each newly-seen app installation into the node_data database.
pub fn make_installation_seen_callback(
    pool: SqlitePool,
) -> Arc<dyn Fn(String, Vec<u8>, RegionId) + Send + Sync> {
    Arc::new(
        move |app_namespace: String, installation_id: Vec<u8>, region_id: RegionId| {
            let pool = pool.clone();
            let region_hex = region_id.to_string();
            tokio::spawn(async move {
                if let Err(e) = AppInstallationsRepo::init()
                    .record_app_installation(&pool, &app_namespace, &installation_id, &region_hex)
                    .await
                {
                    eprintln!("[app_installations] failed to record installation: {}", e);
                }
            });
        },
    )
}
