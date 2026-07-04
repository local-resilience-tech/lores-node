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
            "SELECT app_name, region_node_id, version
            FROM app_installations"
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
            "SELECT app_name, region_node_id, version
            FROM app_installations
            WHERE app_name = ?",
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

    pub async fn find_region_ids_for_apps(
        &self,
        pool: &SqlitePool,
        app_names: &[String],
        node_id: &str,
    ) -> Result<HashMap<String, String>, sqlx::Error> {
        if app_names.is_empty() {
            return Ok(HashMap::new());
        }

        let placeholders = app_names.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let sql = format!(
            "SELECT ai.app_name, rn.region_id
            FROM app_installations ai
            INNER JOIN region_nodes rn ON ai.region_node_id = rn.id
            WHERE ai.app_name IN ({placeholders}) AND rn.node_id = ?"
        );

        let mut query = sqlx::query(&sql);
        for name in app_names {
            query = query.bind(name);
        }
        query = query.bind(node_id);

        let rows = query.fetch_all(pool).await?;

        let mut map = HashMap::new();
        for row in rows {
            use sqlx::Row;
            let app_name: String = row.try_get("app_name")?;
            let region_id: String = row.try_get("region_id")?;
            map.insert(app_name, region_id);
        }

        Ok(map)
    }
}
