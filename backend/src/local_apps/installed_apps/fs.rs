use super::AppIdentifier;
use anyhow::Result;
use std::{
    env,
    fs::{self, DirEntry},
    path::PathBuf,
};

use crate::projections::entities::LocalApp;

lazy_static! {
    pub static ref APPS_PATH: String =
        env::var("APPS_PATH").unwrap_or_else(|_| "../apps".to_string());
}

pub fn find_installed_apps() -> Vec<LocalApp> {
    find_app_dirs()
        .into_iter()
        .map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.is_some())
        .map(|name| AppIdentifier {
            name: name.unwrap(),
        })
        .map(|app_ref| load_app_config(&app_ref))
        .filter_map(|app| app)
        .collect::<Vec<LocalApp>>()
}

pub fn load_app_config(app_ref: &AppIdentifier) -> Option<LocalApp> {
    let path = app_path(app_ref);
    let config_file_path = path.join("config/app.toml");

    match fs::read_to_string(config_file_path.clone()) {
        Ok(file_contents) => match toml::from_str::<LocalApp>(&file_contents) {
            Ok(contents) => Some(contents),
            Err(e) => {
                eprintln!("Could not parse TOML for `{}`: {}", path.display(), e);
                None
            }
        },
        Err(_) => {
            eprintln!("Could not read file `{}`", config_file_path.display());
            None
        }
    }
}

fn app_path(app_ref: &AppIdentifier) -> PathBuf {
    PathBuf::from(APPS_PATH.clone()).join(&app_ref.name)
}

fn find_app_dirs() -> Vec<DirEntry> {
    println!("Finding apps using path: {}", *APPS_PATH);
    let read_result = fs::read_dir(APPS_PATH.clone());

    match read_result {
        Ok(paths) => paths.filter_map(Result::ok).collect(),
        Err(e) => {
            eprintln!("Error reading apps directory: {}", e);
            vec![]
        }
    }
}
