use crate::data::entities::{LocalApp, LocalAppInstallation};

pub fn build_local_app_installations(local_apps: Vec<LocalApp>) -> Vec<LocalAppInstallation> {
    local_apps
        .into_iter()
        .map(|app| {
            let region_id = app.bound_to_region_id.clone();
            LocalAppInstallation { app, region_id }
        })
        .collect()
}
