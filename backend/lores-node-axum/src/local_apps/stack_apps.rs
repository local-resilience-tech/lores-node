pub use coop_cloud_docker_apps::coop_cloud_apps;

use crate::data::entities::{LocalApp, LocalAppSource, NodeAppUrl};

pub fn find_deployed_local_apps() -> Vec<LocalApp> {
    coop_cloud_apps()
        .into_iter()
        .map(|app| LocalApp {
            name: app.name,
            version: app.version,
            url: app.url.map(|u| NodeAppUrl {
                internet_url: u.internet_url,
                local_network_url: u.local_network_url,
            }),
            source: LocalAppSource::Docker,
            instance_id: app.instance_id,
            bound_to_region_id: None,
        })
        .collect()
}
