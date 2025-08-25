use sqlx::SqlitePool;
use std::collections::HashMap;

use crate::data::entities::{AppInstallation, RegionAppWithInstallations};

pub struct AppsReadRepo {}

impl AppsReadRepo {
    pub fn init() -> Self {
        AppsReadRepo {}
    }

    // pub async fn all(&self, pool: &SqlitePool) -> Result<Vec<RegionApp>, sqlx::Error> {
    //     sqlx::query_as!(RegionApp, "SELECT name FROM apps")
    //         .fetch_all(pool)
    //         .await
    // }

    pub async fn all_with_installations(
        &self,
        pool: &SqlitePool,
    ) -> Result<Vec<RegionAppWithInstallations>, sqlx::Error> {
        let installations = sqlx::query_as!(
            AppInstallation,
            "SELECT app_name, node_id, version FROM app_installations"
        )
        .fetch_all(pool)
        .await?;

        // Group installations by app name
        let mut app_map: HashMap<String, Vec<AppInstallation>> = HashMap::new();
        for installation in installations {
            app_map
                .entry(installation.app_name.clone())
                .or_insert_with(Vec::new)
                .push(installation);
        }

        // Convert to RegionAppWithInstallations
        let region_apps: Vec<RegionAppWithInstallations> = app_map
            .into_iter()
            .map(|(name, installations)| RegionAppWithInstallations {
                name,
                installations,
            })
            .collect();

        Ok(region_apps)
    }

    pub async fn find_with_installations(
        &self,
        pool: &SqlitePool,
        name: String,
    ) -> Result<Option<RegionAppWithInstallations>, sqlx::Error> {
        let installations = sqlx::query_as!(
            AppInstallation,
            "SELECT app_name, node_id, version FROM app_installations WHERE app_name = ?",
            name
        )
        .fetch_all(pool)
        .await?;

        if installations.is_empty() {
            return Ok(None);
        }

        let region_app = RegionAppWithInstallations {
            name: name.clone(),
            installations,
        };

        Ok(Some(region_app))
    }
}
