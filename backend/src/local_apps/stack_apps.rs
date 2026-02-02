use std::collections::HashMap;

use crate::{
    data::entities::{LocalApp, NodeAppUrl},
    docker::{
        docker_service::docker_service_inspect,
        docker_stack::{docker_stack_ls, docker_stack_services, DockerStackServicesResult},
        DockerStack,
    },
    local_apps::coop_cloud::service_labels::CoopCloudServiceLabels,
};

pub fn find_deployed_local_apps() -> Vec<LocalApp> {
    let deployed_stacks = docker_stack_ls().unwrap_or_default();

    let local_apps = deployed_stacks
        .into_iter()
        .filter_map(|stack| build_app_details(&stack).ok())
        .collect();

    local_apps
}

fn build_app_details(stack: &DockerStack) -> Result<LocalApp, anyhow::Error> {
    let labels = get_app_service_labels(&stack.name)?;

    Ok(LocalApp {
        name: get_app_name(stack),
        version: labels.version(),
        url: Some(NodeAppUrl {
            internet_url: app_url(labels.host()),
            local_network_url: None,
        }),
    })
}

fn get_app_service_labels(stack_name: &str) -> Result<CoopCloudServiceLabels, anyhow::Error> {
    let services = docker_stack_services(stack_name)?;
    let service = get_app_service_from_list(&services).ok_or_else(|| {
        anyhow::anyhow!(
            "App service not found in stack services for stack: {}",
            stack_name
        )
    })?;

    get_service_lablels(&service.name)
}

fn get_service_lablels(service_id: &str) -> Result<CoopCloudServiceLabels, anyhow::Error> {
    let properties = docker_service_inspect(service_id).map_err(|e| {
        eprintln!("Error inspecting service {}: {:?}", service_id, e);
        e
    })?;

    let labels: HashMap<String, String> = properties.spec.labels.unwrap_or_default();
    let service_labels = CoopCloudServiceLabels::new(labels.clone())?;

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
