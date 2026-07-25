use tracing::warn;

use crate::apps::build_coop_cloud_app;
use crate::docker::docker_stack::docker_stack_ls;

mod apps;
mod docker;
mod service_labels;

pub use apps::{AppUrl, CoopCloudApp};

pub fn find_coop_cloud_apps() -> Vec<CoopCloudApp> {
    let deployed_stacks = docker_stack_ls().unwrap_or_else(|e| {
        warn!("Error listing docker stacks: {:?}", e);
        vec![]
    });

    deployed_stacks
        .into_iter()
        .filter_map(|stack| build_coop_cloud_app(&stack).ok())
        .collect()
}
