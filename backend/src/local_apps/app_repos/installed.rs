use std::{env, path::PathBuf};

use super::{
    super::app_definitions::{fs::app_definition_at_path, AppDefinition},
    AppRepo, AppRepoReference,
};

lazy_static! {
    pub static ref APP_REPOS_PATH: String = env::var("APP_REPOS_PATH")
        .unwrap_or_else(|_| panic!("APP_REPOS_PATH environment variable is not set"));
}

pub fn list_installed_app_repos() -> Vec<AppRepo> {
    list_installed_app_repo_references()
        .into_iter()
        .filter_map(|repo_ref| app_repo_at_reference(&repo_ref))
        .collect()
}

fn list_installed_app_repo_paths() -> Vec<PathBuf> {
    let path = app_repos_path();
    if !path.exists() {
        eprint!("App repos path does not exist: {}", path.display());
        return vec![];
    }

    std::fs::read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|p| p.is_dir())
        .collect()
}

pub fn app_repo_at_reference(repo_ref: &AppRepoReference) -> Option<AppRepo> {
    let apps = app_definitions_in_repo(repo_ref);
    Some(AppRepo {
        name: repo_ref.name.clone(),
        apps,
    })
}

pub fn list_installed_app_repo_references() -> Vec<AppRepoReference> {
    list_installed_app_repo_paths()
        .into_iter()
        .filter_map(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| AppRepoReference {
                    name: name.to_string(),
                })
        })
        .collect()
}

fn app_definitions_in_repo(repo_ref: &AppRepoReference) -> Vec<AppDefinition> {
    app_definitions_in_path(app_repo_path(repo_ref))
}

fn app_definitions_in_path(path: PathBuf) -> Vec<AppDefinition> {
    let app_paths = app_paths_in_repo_path(&path);
    app_paths
        .into_iter()
        .filter_map(|app_path| app_definition_at_path(&app_path))
        .collect()
}

fn app_paths_in_repo_path(path: &PathBuf) -> Vec<PathBuf> {
    if !path.exists() {
        eprintln!("App repo path does not exist: {}", path.display());
        return vec![];
    }

    std::fs::read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|p| p.is_dir())
        .collect()
}

fn app_repo_path(repo_ref: &AppRepoReference) -> PathBuf {
    app_repos_path().join(&repo_ref.name)
}

pub fn app_repos_path() -> PathBuf {
    PathBuf::from(&*APP_REPOS_PATH)
}
