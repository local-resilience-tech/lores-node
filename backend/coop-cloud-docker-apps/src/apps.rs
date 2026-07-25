use std::collections::HashMap;
use tracing::warn;

use crate::docker::{
    DockerStack,
    docker_service::docker_service_inspect,
    docker_stack::{DockerStackServicesResult, docker_stack_services},
};
use crate::service_labels::CoopCloudServiceLabels;

#[derive(Debug, Clone)]
pub struct AppUrl {
    pub internet_url: Option<String>,
    pub local_network_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CoopCloudApp {
    pub name: String,
    pub version: String,
    pub url: Option<AppUrl>,
    pub instance_id: Option<String>,
}

pub fn build_coop_cloud_app(stack: &DockerStack) -> Result<CoopCloudApp, anyhow::Error> {
    let labels = get_app_service_labels(&stack.name)?;

    Ok(CoopCloudApp {
        name: get_app_name(stack),
        version: labels.version(),
        url: Some(AppUrl {
            internet_url: app_url(labels.host()),
            local_network_url: None,
        }),
        instance_id: labels.lores_instance_id(),
    })
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

fn app_url(host: Option<String>) -> Option<String> {
    host.map(|h| format!("https://{}", h))
}

fn get_app_name(stack: &DockerStack) -> String {
    stack
        .name
        .split('_')
        .next()
        .unwrap_or(&stack.name)
        .to_string()
}
