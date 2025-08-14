use anyhow::{Error, Result};
use std::{
    env,
    fs::{self},
    path::PathBuf,
};

use super::{
    super::{
        app_repos::{
            fs::app_repo_from_app_name,
            git_app_repos::{with_checked_out_app_version, CheckoutAppVersionError},
            AppRepoAppReference,
        },
        shared::app_definitions::{self, config::app_config_from_string, AppVersionDefinition},
    },
    AppReference,
};
use crate::projections::entities::{LocalApp, LocalAppInstallStatus};

lazy_static! {
    pub static ref APPS_PATH: String =
        env::var("APPS_PATH").unwrap_or_else(|_| "../apps".to_string());
}

pub fn find_installed_apps() -> Vec<AppVersionDefinition> {
    app_definitions::fs::app_definitions_in_path(PathBuf::from(APPS_PATH.clone()))
}

pub fn load_app_config(app_ref: &AppReference) -> Option<LocalApp> {
    let path = app_path(app_ref);
    let config_file_path = path.join("config/app.toml");

    match fs::read_to_string(config_file_path.clone()) {
        Ok(file_contents) => match app_config_from_string(file_contents) {
            Ok(app_definition) => Some(LocalApp {
                name: app_definition.name.clone(),
                version: app_definition.version,
                status: LocalAppInstallStatus::Installed,
                repo_name: app_repo_from_app_name(app_definition.name.as_str())
                    .map(|repo| repo.repo_name),
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

#[derive(Debug)]
#[allow(dead_code)]
pub enum InstallAppVersionError {
    InUse,
    FileSystemError,
    LoadingAppError,
    CheckoutError,
    OtherError,
}

pub fn install_app_definition(
    source: &AppRepoAppReference,
    target: &AppReference,
) -> Result<LocalApp, InstallAppVersionError> {
    with_checked_out_app_version(source, |source_path| {
        println!(
            "In install callback `{}` to `{}`",
            source_path.display(),
            app_path(target).display()
        );

        let target_path = app_path(target);

        copy_app_files(&source_path, &target_path)
            .map_err(|_| CheckoutAppVersionError::CallbackError(Error::msg("FileSystemError")))?;

        Ok(())
    })
    .map_err(|e| match e {
        CheckoutAppVersionError::InUse => InstallAppVersionError::InUse,
        CheckoutAppVersionError::CallbackError(inner) => match inner.to_string().as_str() {
            "FileSystemError" => InstallAppVersionError::FileSystemError,
            _ => InstallAppVersionError::OtherError,
        },
        _ => InstallAppVersionError::CheckoutError,
    })?;

    load_app_config(target).ok_or(InstallAppVersionError::LoadingAppError)
}

pub fn compose_file_path(app_ref: &AppReference) -> PathBuf {
    app_path(app_ref).join("compose.yml")
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
