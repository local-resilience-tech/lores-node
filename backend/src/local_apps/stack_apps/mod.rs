use crate::{
    docker::{
        docker_stack::{docker_stack_deploy, docker_stack_ls, docker_stack_rm},
        DockerStack,
    },
    projections::entities::{LocalApp, LocalAppInstallStatus},
};

use super::installed_apps::{self, fs::compose_file_path, AppReference};

pub fn find_deployed_local_apps() -> Vec<LocalApp> {
    let app_definitions = installed_apps::fs::find_installed_apps();
    let deployed_stacks = docker_stack_ls().unwrap_or_default();

    let local_apps = app_definitions
        .into_iter()
        .map(|app_definition| LocalApp {
            name: app_definition.name,
            version: app_definition.version,
            status: LocalAppInstallStatus::Installed,
        })
        .collect();

    let local_apps = update_app_statuses_from_stacks(&local_apps, &deployed_stacks);
    local_apps
}

pub fn deploy_local_app(app_ref: &AppReference) -> Result<LocalApp, anyhow::Error> {
    let compose_file_path = compose_file_path(app_ref);

    docker_stack_deploy(&app_ref.app_name, &compose_file_path)?;

    find_local_app(&app_ref)
}

pub fn remove_local_app(app_ref: &AppReference) -> Result<LocalApp, anyhow::Error> {
    docker_stack_rm(&app_ref.app_name)?;
    find_local_app(&app_ref)
}

fn find_local_app(app_ref: &AppReference) -> Result<LocalApp, anyhow::Error> {
    let app = installed_apps::fs::load_app_config(app_ref)
        .ok_or_else(|| anyhow::anyhow!("Failed to load app config for {}", app_ref.app_name))?;
    let deployed_stacks = docker_stack_ls().unwrap_or_default();

    let updated_app = update_app_status_from_stacks(&app, &deployed_stacks);

    Ok(updated_app)
}

fn update_app_statuses_from_stacks(
    apps: &Vec<LocalApp>,
    deployed_stacks: &[DockerStack],
) -> Vec<LocalApp> {
    apps.iter()
        .cloned()
        .map(|app| update_app_status_from_stacks(&app, deployed_stacks))
        .collect()
}

fn update_app_status_from_stacks(app: &LocalApp, deployed_stacks: &[DockerStack]) -> LocalApp {
    let mut updated_app = app.clone();
    if app_in_docker_stacks(&app.name, deployed_stacks) {
        updated_app.status = LocalAppInstallStatus::StackDeployed;
    }
    updated_app
}

fn app_in_docker_stacks(app_name: &str, deployed_stacks: &[DockerStack]) -> bool {
    deployed_stacks.iter().any(|stack| stack.name == app_name)
}
