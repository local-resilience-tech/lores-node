use anyhow::{Error, Result};
use std::fs::{self};

use super::{
    super::{
        app_repos::{
            fs::app_repo_from_app_name,
            git_app_repos::{with_checked_out_app_version, CheckoutAppVersionError},
            AppRepoAppReference,
        },
        shared::app_definitions::{parse_app_definition, AppVersionDefinition},
    },
    app_folder::AppFolder,
    apps_folder::AppsFolder,
    AppReference,
};
use crate::data::entities::{LocalApp, LocalAppInstallStatus};

pub fn find_installed_apps() -> Vec<AppVersionDefinition> {
    let apps_folder = AppsFolder::new();
    apps_folder.app_definitions()
}

pub fn load_app_config(app_ref: &AppReference) -> Option<LocalApp> {
    let app_folder = AppFolder::new(app_ref.clone());
    let config_file_path = app_folder.config_file_path();

    match fs::read_to_string(config_file_path.clone()) {
        Ok(file_contents) => match parse_app_definition(file_contents) {
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
        let app_folder = AppFolder::new(target.clone());

        println!(
            "In install callback `{:?}` to `{:?}`",
            source_path, app_folder.app_ref.app_name
        );

        app_folder
            .ensure_exists()
            .map_err(|_| CheckoutAppVersionError::CallbackError(Error::msg("FileSystemError")))?;

        app_folder
            .copy_in_version(&source_path, &source.version)
            .map_err(|_| CheckoutAppVersionError::CallbackError(Error::msg("FileSystemError")))?;

        app_folder
            .make_current_version(&source.version)
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
