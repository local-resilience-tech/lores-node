use std::{env, path::PathBuf};

use super::{
    super::shared::app_definitions::{fs::app_definitions_in_path, AppDefinition},
    git::git_origin_url,
    AppRepo, AppRepoAppReference, AppRepoReference, AppRepoSource,
};

lazy_static! {
    pub static ref APP_REPOS_PATH: String = env::var("APP_REPOS_PATH")
        .unwrap_or_else(|_| panic!("APP_REPOS_PATH environment variable is not set"));
}

pub fn list_installed_app_repos() -> Vec<AppRepo> {
    list_installed_app_repo_sources()
        .into_iter()
        .filter_map(|repo_src| app_repo_at_source(&repo_src))
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

pub fn app_repo_at_source(repo_src: &AppRepoSource) -> Option<AppRepo> {
    let repo_ref = AppRepoReference {
        repo_name: repo_src.name.clone(),
    };
    let apps = app_definitions_in_repo(&repo_ref);
    Some(AppRepo {
        name: repo_src.name.clone(),
        git_url: repo_src.git_url.clone(),
        apps,
    })
}

pub fn list_installed_app_repo_sources() -> Vec<AppRepoSource> {
    list_installed_app_repo_paths()
        .into_iter()
        .filter_map(|path| app_repo_source_from_path(&path))
        .collect()
}

fn app_repo_source_from_path(path: &PathBuf) -> Option<AppRepoSource> {
    let name = path.file_name()?.to_str()?;
    let app_ref = AppRepoReference {
        repo_name: name.to_string(),
    };
    match git_origin_url(&app_ref) {
        Ok(url) => Some(AppRepoSource {
            name: name.to_string(),
            git_url: url,
        }),
        Err(_) => {
            eprintln!("Failed to get git URL for app repo: {}", name);
            None
        }
    }
}
fn app_definitions_in_repo(repo_ref: &AppRepoReference) -> Vec<AppDefinition> {
    app_definitions_in_path(app_repo_path(repo_ref))
}

pub fn app_repo_app_path(app_ref: &AppRepoAppReference) -> PathBuf {
    app_repo_path(&app_ref.repo_ref()).join(&app_ref.app_name)
}

pub fn app_repo_path(repo_ref: &AppRepoReference) -> PathBuf {
    app_repos_path().join(&repo_ref.repo_name)
}

pub fn app_repos_path() -> PathBuf {
    PathBuf::from(&*APP_REPOS_PATH)
}
