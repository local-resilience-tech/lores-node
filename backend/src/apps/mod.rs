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
        .map(|name| load_app_config(name.unwrap()))
        .filter_map(|app| app)
        .collect::<Vec<LocalApp>>()
}

pub fn load_app_config(app_name: String) -> Option<LocalApp> {
    let path = app_path(app_name);
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

fn app_path(app_name: String) -> PathBuf {
    PathBuf::from(APPS_PATH.clone()).join(app_name)
}

fn find_app_dirs() -> Vec<DirEntry> {
    println!("Finding apps using path: {}", *APPS_PATH);
    let paths = fs::read_dir(APPS_PATH.clone()).unwrap();
    paths.filter_map(Result::ok).collect()
}
