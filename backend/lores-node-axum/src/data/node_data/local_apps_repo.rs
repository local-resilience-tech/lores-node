use sqlx::{Sqlite, SqlitePool};

use crate::data::entities::{LocalApp, LocalAppSource, NodeAppUrl};

#[derive(sqlx::FromRow)]
struct LocalAppRow {
    name: String,
    version: String,
    internet_url: Option<String>,
    local_network_url: Option<String>,
    instance_id: Option<String>,
}

impl From<LocalAppRow> for LocalApp {
    fn from(row: LocalAppRow) -> Self {
        let url = if row.internet_url.is_none() && row.local_network_url.is_none() {
            None
        } else {
            Some(NodeAppUrl {
                internet_url: row.internet_url,
                local_network_url: row.local_network_url,
            })
        };

        LocalApp {
            name: row.name,
            version: row.version,
            url,
            source: LocalAppSource::Db,
            instance_id: row.instance_id,
        }
    }
}

pub struct LocalAppsRepo {}

impl LocalAppsRepo {
    pub fn init() -> Self {
        LocalAppsRepo {}
    }

    pub async fn all(&self, pool: &SqlitePool) -> Result<Vec<LocalApp>, sqlx::Error> {
        let rows = sqlx::query_as::<Sqlite, LocalAppRow>(
            "
            SELECT name, version, internet_url, local_network_url, instance_id
            FROM local_apps
            ORDER BY name ASC
            ",
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(LocalApp::from).collect())
    }

    pub async fn create(&self, pool: &SqlitePool, app: &LocalApp) -> Result<LocalApp, sqlx::Error> {
        let internet_url = app.url.as_ref().and_then(|url| url.internet_url.clone());
        let local_network_url = app
            .url
            .as_ref()
            .and_then(|url| url.local_network_url.clone());

        sqlx::query::<Sqlite>(
            "
            INSERT INTO local_apps (name, version, internet_url, local_network_url, instance_id)
            VALUES (?, ?, ?, ?, ?)
            ",
        )
        .bind(&app.name)
        .bind(&app.version)
        .bind(internet_url)
        .bind(local_network_url)
        .bind(&app.instance_id)
        .execute(pool)
        .await?;

        Ok(LocalApp {
            name: app.name.clone(),
            version: app.version.clone(),
            url: app.url.clone(),
            source: LocalAppSource::Db,
            instance_id: app.instance_id.clone(),
        })
    }
}
