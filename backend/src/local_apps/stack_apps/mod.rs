use std::env;

use crate::{
    data::entities::{LocalApp, LocalAppInstallStatus, Node, NodeAppUrl},
    docker::{
        docker_stack::{docker_stack_ls, docker_stack_rm},
        DockerStack,
    },
};

use super::{
    app_repos::fs::app_repo_from_app_name,
    installed_apps::fs::{find_installed_apps, load_local_app_details},
    installed_apps::AppReference,
};

lazy_static! {
    pub static ref HOST_OS_APPS_PATH: String = env::var("HOST_OS_APPS_PATH").unwrap();
}

pub fn find_deployed_local_apps(node: &Node) -> Vec<LocalApp> {
    let apps_details = find_installed_apps();
    let deployed_stacks = docker_stack_ls().unwrap_or_default();

    let local_apps = apps_details
        .into_iter()
        .map(|app_details| LocalApp {
            name: app_details.name.clone(),
            version: app_details.version,
            status: LocalAppInstallStatus::Installed,
            repo_name: app_repo_from_app_name(app_details.name.as_str()).map(|repo| repo.repo_name),
            has_config_schema: app_details.has_config_schema,
            url: None,
        })
        .collect();

    let local_apps = update_app_statuses_from_stacks(&local_apps, node, &deployed_stacks);
    local_apps
}

pub fn remove_local_app(app_ref: &AppReference, node: &Node) -> Result<LocalApp, anyhow::Error> {
    docker_stack_rm(&app_ref.app_name)?;
    find_local_app(&app_ref, node)
}

fn find_local_app(app_ref: &AppReference, node: &Node) -> Result<LocalApp, anyhow::Error> {
    let app = load_local_app_details(app_ref)
        .ok_or_else(|| anyhow::anyhow!("Failed to load app config for {}", app_ref.app_name))?;
    let deployed_stacks = docker_stack_ls().unwrap_or_default();

    let updated_app = update_app_status_from_stacks(&app, node, &deployed_stacks);

    Ok(updated_app)
}

fn update_app_statuses_from_stacks(
    apps: &Vec<LocalApp>,
    node: &Node,
    deployed_stacks: &[DockerStack],
) -> Vec<LocalApp> {
    apps.iter()
        .cloned()
        .map(|app| update_app_status_from_stacks(&app, node, deployed_stacks))
        .collect()
}

fn update_app_status_from_stacks(
    app: &LocalApp,
    node: &Node,
    deployed_stacks: &[DockerStack],
) -> LocalApp {
    let mut updated_app = app.clone();
    if app_in_docker_stacks(&app.name, deployed_stacks) {
        updated_app.status = LocalAppInstallStatus::StackDeployed;
        updated_app.url = Some(NodeAppUrl {
            internet_url: app_url(&app.name, node.domain_on_internet.as_deref()),
            local_network_url: app_url(&app.name, node.domain_on_local_network.as_deref()),
        });
    }
    updated_app
}

fn app_url(app_name: &str, domain: Option<&str>) -> Option<String> {
    domain.map(|d| format!("http://{}.{}", app_name, d))
}

fn app_in_docker_stacks(app_name: &str, deployed_stacks: &[DockerStack]) -> bool {
    deployed_stacks.iter().any(|stack| stack.name == app_name)
}
