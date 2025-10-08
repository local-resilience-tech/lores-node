use crate::{
    data::entities::{LocalApp, LocalAppInstallStatus},
    docker::{
        docker_stack::{docker_stack_compose_and_deploy, docker_stack_ls, docker_stack_rm},
        DockerStack,
    },
};

use self::system_compose_files::SystemComposeFiles;
use super::{
    app_repos::fs::app_repo_from_app_name,
    installed_apps::{self, app_folder::AppFolder, AppReference},
};

mod system_compose_files;

pub fn find_deployed_local_apps() -> Vec<LocalApp> {
    let apps_details = installed_apps::fs::find_installed_apps();
    let deployed_stacks = docker_stack_ls().unwrap_or_default();

    let local_apps = apps_details
        .into_iter()
        .map(|app_details| LocalApp {
            name: app_details.name.clone(),
            version: app_details.version,
            status: LocalAppInstallStatus::Installed,
            repo_name: app_repo_from_app_name(app_details.name.as_str()).map(|repo| repo.repo_name),
            has_config_schema: app_details.has_config_schema,
        })
        .collect();

    let local_apps = update_app_statuses_from_stacks(&local_apps, &deployed_stacks);
    local_apps
}

pub fn deploy_local_app(app_ref: &AppReference) -> Result<LocalApp, anyhow::Error> {
    let app_folder = AppFolder::new(app_ref.clone());
    let system_files = SystemComposeFiles::new(app_folder.apps_folder.system_folder());

    let compose_file_path = app_folder.compose_file_path();
    let system_paths = system_files.ordered_paths()?;

    let all_compose_paths = [vec![compose_file_path], system_paths].concat();

    docker_stack_compose_and_deploy(&app_ref.app_name, &all_compose_paths)?;

    find_local_app(&app_ref)
}

pub fn remove_local_app(app_ref: &AppReference) -> Result<LocalApp, anyhow::Error> {
    docker_stack_rm(&app_ref.app_name)?;
    find_local_app(&app_ref)
}

fn find_local_app(app_ref: &AppReference) -> Result<LocalApp, anyhow::Error> {
    let app = installed_apps::fs::load_local_app_details(app_ref)
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
