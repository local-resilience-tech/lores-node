use std::collections::HashMap;
use tracing::warn;

use crate::coop_cloud_app::build_coop_cloud_app_from_labels;
use crate::service_labels::CoopCloudServiceLabels;
use crate::{
    CoopCloudApp,
    docker::{
        DockerStack,
        docker_service::docker_service_inspect,
        docker_stack::{DockerStackServicesResult, docker_stack_services},
    },
};

pub fn build_coop_cloud_app(stack: &DockerStack) -> Result<CoopCloudApp, anyhow::Error> {
    let labels = get_app_service_labels(&stack.name)?;
    build_coop_cloud_app_from_labels(&labels)
}

fn get_app_service_labels(stack_name: &str) -> Result<CoopCloudServiceLabels, anyhow::Error> {
    let services = docker_stack_services(stack_name).map_err(|e| {
        warn!("Error listing services for stack {}: {:?}", stack_name, e);
        e
    })?;
    let service = get_app_service_from_list(&services).ok_or_else(|| {
        anyhow::anyhow!(
            "App service not found in stack services for stack: {}",
            stack_name
        )
    })?;

    get_service_labels(&service.name)
}

fn get_service_labels(service_id: &str) -> Result<CoopCloudServiceLabels, anyhow::Error> {
    let properties = docker_service_inspect(service_id).map_err(|e| {
        warn!("Error inspecting service {}: {:?}", service_id, e);
        e
    })?;

    let labels: HashMap<String, String> = properties.spec.labels.unwrap_or_default();
    let service_labels = CoopCloudServiceLabels::new(labels)?;

    Ok(service_labels)
}

fn get_app_service_from_list(
    services: &Vec<DockerStackServicesResult>,
) -> Option<&DockerStackServicesResult> {
    services
        .iter()
        .find(|service| service.name.ends_with("_app"))
}
