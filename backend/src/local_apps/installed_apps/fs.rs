use anyhow::Result;
use std::{
    env,
    fs::{self},
    path::PathBuf,
};

use super::{
    super::{
        app_repos::{self, AppRepoAppReference},
        shared::app_definitions,
    },
    AppReference,
};
use crate::{
    local_apps::shared::app_definitions::config::app_config_from_string,
    projections::entities::LocalApp,
};

lazy_static! {
    pub static ref APPS_PATH: String =
        env::var("APPS_PATH").unwrap_or_else(|_| "../apps".to_string());
}

pub fn find_installed_apps() -> Vec<LocalApp> {
    app_definitions::fs::app_definitions_in_path(PathBuf::from(APPS_PATH.clone()))
        .into_iter()
        .map(|app_definition| LocalApp {
            name: app_definition.name.clone(),
            version: app_definition.version.clone(),
        })
        .collect::<Vec<LocalApp>>()
}

pub fn load_app_config(app_ref: &AppReference) -> Option<LocalApp> {
    let path = app_path(app_ref);
    let config_file_path = path.join("config/app.toml");

    match fs::read_to_string(config_file_path.clone()) {
        Ok(file_contents) => match app_config_from_string(file_contents) {
            Ok(app_definition) => Some(LocalApp {
                name: app_definition.name,
                version: app_definition.version,
            }),
            Err(_) => {
                eprintln!(
                    "Failed to parse app definition at `{}`",
                    config_file_path.display()
                );
                None
            }
        },
        Err(_) => {
            eprintln!("Could not read file `{}`", config_file_path.display());
            None
        }
    }
}

pub fn install_app_definition(
    source: &AppRepoAppReference,
    target: &AppReference,
) -> Result<LocalApp, ()> {
    let source_path = app_repos::fs::app_repo_app_path(source);
    let target_path = app_path(target);
    copy_app_files(&source_path, &target_path)?;
    load_app_config(target).ok_or(())
}

fn app_path(app_ref: &AppReference) -> PathBuf {
    apps_path().join(&app_ref.app_name)
}

fn apps_path() -> PathBuf {
    PathBuf::from(APPS_PATH.clone())
}

fn copy_app_files(source: &PathBuf, target: &PathBuf) -> Result<(), ()> {
    match copy_dir::copy_dir(source, target) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!(
                "Failed to copy files from `{}` to `{}`: {}",
                source.display(),
                target.display(),
                e
            );
            Err(())
        }
    }
}
