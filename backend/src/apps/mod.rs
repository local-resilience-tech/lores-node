use anyhow::Result;
use std::{
    env,
    fs::{self, DirEntry},
};

use crate::projections::entities::App;

lazy_static! {
    pub static ref APPS_PATH: String =
        env::var("APPS_PATH").unwrap_or_else(|_| "../apps".to_string());
}

pub fn find_installed_apps() -> Vec<App> {
    find_app_dirs()
        .into_iter()
        .map(load_app_config)
        .filter_map(|app| app)
        .collect::<Vec<App>>()
}

fn load_app_config(path: DirEntry) -> Option<App> {
    let config_file_path = path.path().join("config/app.toml");

    match fs::read_to_string(config_file_path.clone()) {
        Ok(file_contents) => match toml::from_str::<App>(&file_contents) {
            Ok(contents) => Some(contents),
            Err(e) => {
                eprintln!(
                    "Could not parse TOML for `{}`: {}",
                    path.path().display(),
                    e
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

fn find_app_dirs() -> Vec<DirEntry> {
    println!("Finding apps using path: {}", *APPS_PATH);
    let paths = fs::read_dir(APPS_PATH.clone()).unwrap();
    paths.filter_map(Result::ok).collect()
}
