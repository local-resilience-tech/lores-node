use crate::projections::entities::{LocalApp, LocalAppInstallStatus};

use super::installed_apps;

pub fn find_deployed_local_apps() -> Vec<LocalApp> {
    let app_definitions = installed_apps::fs::find_installed_apps();

    app_definitions
        .into_iter()
        .map(|app_definition| LocalApp {
            name: app_definition.name,
            version: app_definition.version,
            status: LocalAppInstallStatus::Installed,
        })
        .collect()
}
