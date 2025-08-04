use crate::{
    docker::{docker_stack::docker_stack_ls, DockerStack},
    projections::entities::{LocalApp, LocalAppInstallStatus},
};

use super::installed_apps;

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

    let local_apps = update_app_status_from_stacks(&local_apps, &deployed_stacks);
    local_apps
}

fn update_app_status_from_stacks(
    apps: &Vec<LocalApp>,
    deployed_stacks: &[DockerStack],
) -> Vec<LocalApp> {
    apps.iter()
        .cloned()
        .map(|mut app| {
            if app_in_docker_stacks(&app.name, deployed_stacks) {
                app.status = LocalAppInstallStatus::StackDeployed;
            }
            app
        })
        .collect()
}

fn app_in_docker_stacks(app_name: &str, deployed_stacks: &[DockerStack]) -> bool {
    deployed_stacks.iter().any(|stack| stack.name == app_name)
}
