use std::fs::{self};

use super::{
    super::shared::app_definitions::parse_app_definition,
    app_folder::{AppFolder, InstalledAppDetails},
    apps_folder::AppsFolder,
    AppReference,
};
use crate::data::entities::{LocalApp, LocalAppInstallStatus};

pub fn find_installed_apps() -> Vec<InstalledAppDetails> {
    let apps_folder = AppsFolder::new();
    apps_folder.apps()
}

pub fn load_local_app_details(app_ref: &AppReference) -> Option<LocalApp> {
    let app_folder = AppFolder::new(app_ref.clone());
    let config_file_path = app_folder.app_definition_file_path();

    match fs::read_to_string(config_file_path.clone()) {
        Ok(file_contents) => match parse_app_definition(file_contents) {
            Ok(app_definition) => Some(LocalApp {
                name: app_definition.name.clone(),
                version: app_definition.version,
                status: LocalAppInstallStatus::Installed,
                has_config_schema: app_folder.has_config_schema(),
                url: None,
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
