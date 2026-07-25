use crate::{
    data::entities::{LocalApp, LocalAppSource},
    local_apps::coop_cloud_apps::find_coop_cloud_apps,
};

pub fn find_deployed_local_apps() -> Vec<LocalApp> {
    find_coop_cloud_apps()
        .into_iter()
        .map(|app| LocalApp {
            name: app.name,
            version: app.version,
            url: app.url,
            source: LocalAppSource::Docker,
            instance_id: app.instance_id,
            bound_to_region_id: None,
        })
        .collect()
}
