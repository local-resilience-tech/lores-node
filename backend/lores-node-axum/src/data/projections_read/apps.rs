use sqlx::SqlitePool;
use std::collections::HashMap;

use crate::data::entities::{AppInstallation, RegionAppWithInstallations};

struct InstallationRow {
    app_name: String,
    region_node_id: i64,
    version: String,
    region_id: String,
}

impl InstallationRow {
    fn into_installation(self) -> AppInstallation {
        AppInstallation {
            app_name: self.app_name,
            region_node_id: self.region_node_id,
            version: self.version,
        }
    }
}

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
        let rows = sqlx::query_as!(
            InstallationRow,
            "SELECT app_installations.app_name, app_installations.region_node_id, app_installations.version, region_nodes.region_id
            FROM app_installations
            INNER JOIN region_nodes ON app_installations.region_node_id = region_nodes.id"
        )
        .fetch_all(pool)
        .await?;

        let mut app_map: HashMap<String, (String, Vec<AppInstallation>)> = HashMap::new();
        for row in rows {
            let (_, installations) = app_map
                .entry(row.app_name.clone())
                .or_insert_with(|| (row.region_id.clone(), Vec::new()));
            installations.push(row.into_installation());
        }

        let region_apps = app_map
            .into_iter()
            .map(
                |(name, (region_id, installations))| RegionAppWithInstallations {
                    name,
                    region_id,
                    installations,
                },
            )
            .collect();

        Ok(region_apps)
    }

    pub async fn find_with_installations(
        &self,
        pool: &SqlitePool,
        name: String,
    ) -> Result<Option<RegionAppWithInstallations>, sqlx::Error> {
        let rows = sqlx::query_as!(
            InstallationRow,
            "SELECT app_installations.app_name, app_installations.region_node_id, app_installations.version, region_nodes.region_id
            FROM app_installations
            INNER JOIN region_nodes ON app_installations.region_node_id = region_nodes.id
            WHERE app_installations.app_name = ?",
            name
        )
        .fetch_all(pool)
        .await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let region_id = rows[0].region_id.clone();
        let installations = rows.into_iter().map(|r| r.into_installation()).collect();

        let region_app = RegionAppWithInstallations {
            name,
            region_id,
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
