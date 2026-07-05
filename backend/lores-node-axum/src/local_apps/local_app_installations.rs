use sqlx::SqlitePool;

use crate::data::{
    entities::{LocalApp, LocalAppInstallation},
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
