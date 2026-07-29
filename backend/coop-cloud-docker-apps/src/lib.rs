use tracing::warn;

use crate::docker::docker_stack::docker_stack_ls;

mod apps;
mod coop_cloud_app;
mod docker;
mod service_labels;

pub use apps::build_coop_cloud_app;
pub use coop_cloud_app::{AppUrl, CoopCloudApp, LoResApp};
pub use docker::{
    DockerService, DockerStack, DockerStackWithServices, docker_stacks_with_services,
};

pub fn coop_cloud_apps() -> Vec<CoopCloudApp> {
    let deployed_stacks = docker_stack_ls().unwrap_or_else(|e| {
        warn!("Error listing docker stacks: {:?}", e);
        vec![]
    });

    deployed_stacks
        .into_iter()
        .filter_map(|stack| build_coop_cloud_app(&stack).ok())
        .collect()
}
