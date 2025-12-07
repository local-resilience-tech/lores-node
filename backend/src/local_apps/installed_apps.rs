use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::data::entities::{LocalApp, LocalAppInstallStatus};

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct AppReference {
    pub app_name: String,
}

pub struct InstalledAppDetails {
    pub name: String,
    pub version: String,
}

pub fn dummy_local_apps() -> Vec<LocalApp> {
    vec![
        LocalApp {
            name: "example_app".to_string(),
            version: "1.0.0".to_string(),
            status: LocalAppInstallStatus::Installed,
            url: None,
        },
        LocalApp {
            name: "another_app".to_string(),
            version: "2.1.3".to_string(),
            status: LocalAppInstallStatus::Installed,
            url: None,
        },
    ]
}

pub fn find_installed_apps() -> Vec<InstalledAppDetails> {
    dummy_local_apps()
        .into_iter()
        .map(|app| InstalledAppDetails {
            name: app.name,
            version: app.version,
        })
        .collect()
}

pub fn load_local_app_details(app_ref: &AppReference) -> Option<LocalApp> {
    dummy_local_apps()
        .into_iter()
        .find(|app| app.name == app_ref.app_name)
}
