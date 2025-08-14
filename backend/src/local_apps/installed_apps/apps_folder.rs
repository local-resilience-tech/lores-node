use std::{env, path::PathBuf};

use super::{
    super::shared::app_definitions::AppVersionDefinition, app_folder::AppFolder, AppReference,
};

lazy_static! {
    pub static ref APPS_PATH: String =
        env::var("APPS_PATH").unwrap_or_else(|_| "../apps".to_string());
}

pub struct AppsFolder {
    path: PathBuf,
}

impl AppsFolder {
    pub fn new() -> Self {
        Self { path: apps_path() }
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn app_definitions(&self) -> Vec<AppVersionDefinition> {
        self.app_folders()
            .into_iter()
            .filter_map(|app_folder| app_folder.app_version_definition())
            .collect()
    }

    fn app_folders(&self) -> Vec<AppFolder> {
        self.app_names()
            .into_iter()
            .map(|app_name| AppFolder::new(AppReference::new(app_name)))
            .collect()
    }

    fn app_names(&self) -> Vec<String> {
        self.app_paths()
            .into_iter()
            .filter_map(|p| p.file_name().and_then(|n| n.to_str()).map(String::from))
            .collect()
    }

    fn app_paths(&self) -> Vec<PathBuf> {
        println!("Finding apps in path: {:?}", self.path);

        std::fs::read_dir(&self.path)
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|p| p.is_dir())
            .filter(|p| p.file_name().and_then(|n| n.to_str()) != Some(".git"))
            .collect()
    }
}

pub fn apps_path() -> PathBuf {
    PathBuf::from(APPS_PATH.clone())
}
