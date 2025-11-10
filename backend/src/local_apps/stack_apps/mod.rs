use std::{collections::HashMap, env};

use crate::{
    data::entities::{LocalApp, LocalAppInstallStatus, Node, NodeAppUrl},
    docker::{
        docker_compose::{docker_compose_interpolate, docker_compose_merge_files_no_interpolate},
        docker_stack::{docker_stack_deploy, docker_stack_ls, docker_stack_rm},
        DockerStack,
    },
};

use self::system_compose_files::SystemComposeFiles;
use super::{
    app_repos::fs::app_repo_from_app_name,
    installed_apps::{self, app_folder::AppFolder, AppReference},
};

mod system_compose_files;

lazy_static! {
    pub static ref HOST_OS_APPS_PATH: String = env::var("HOST_OS_APPS_PATH").unwrap();
}

pub fn find_deployed_local_apps(node: &Node) -> Vec<LocalApp> {
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
            url: None,
        })
        .collect();

    let local_apps = update_app_statuses_from_stacks(&local_apps, node, &deployed_stacks);
    local_apps
}

pub fn deploy_local_app(app_ref: &AppReference, node: &Node) -> Result<LocalApp, anyhow::Error> {
    let app_folder = AppFolder::new(app_ref.clone());
    app_folder.ensure_exists().map_err(|_| {
        anyhow::anyhow!(
            "Failed to ensure app folder exists for {}",
            app_ref.app_name
        )
    })?;

    let system_files = SystemComposeFiles::new(app_folder.apps_folder.system_folder());
    system_files.ensure_system_compose_files()?;

    let setup_env_vars = HashMap::from([
        (
            "HOST_OS_APP_CONFIG_DIR".to_string(),
            app_folder
                .config_dir_path(Some(HOST_OS_APPS_PATH.clone()))
                .to_string_lossy()
                .to_string(),
        ),
        ("LORES_APP_NAME".to_string(), app_ref.app_name.clone()),
    ]);

    let deploy_env_vars = HashMap::from([
        (
            "NODE_LOCAL_DOMAIN".to_string(),
            node.domain_on_local_network
                .clone()
                .unwrap_or("localhost".to_string()),
        ),
        (
            "NODE_INTERNET_DOMAIN".to_string(),
            node.domain_on_internet
                .clone()
                .unwrap_or("localhost".to_string()),
        ),
    ]);

    println!(
        "About to merge compose files to: {}",
        app_folder.merged_compose_file_path().display()
    );

    print!("Setup env vars: ");
    for (key, value) in &setup_env_vars {
        print!("{}={}, ", key, value);
    }
    println!();

    docker_compose_merge_files_no_interpolate(
        vec![
            app_folder.compose_file_path(),
            system_files.reset_path(),
            system_files.setup_path(),
        ],
        &app_folder.merged_compose_file_path(),
    )?;

    let mut env_vars = setup_env_vars.clone();
    env_vars.extend(deploy_env_vars.clone());

    docker_compose_interpolate(
        &app_folder.merged_compose_file_path(),
        &app_folder.interpolated_compose_file_path(),
        &env_vars,
    )?;

    docker_stack_deploy(
        &app_ref.app_name,
        &app_folder.interpolated_compose_file_path(),
    )?;

    find_local_app(&app_ref, node)
}

pub fn remove_local_app(app_ref: &AppReference, node: &Node) -> Result<LocalApp, anyhow::Error> {
    docker_stack_rm(&app_ref.app_name)?;
    find_local_app(&app_ref, node)
}

fn find_local_app(app_ref: &AppReference, node: &Node) -> Result<LocalApp, anyhow::Error> {
    let app = installed_apps::fs::load_local_app_details(app_ref)
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
