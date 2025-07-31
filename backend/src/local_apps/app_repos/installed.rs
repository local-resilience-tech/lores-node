use std::{env, path::PathBuf};

use super::AppRepo;

lazy_static! {
    pub static ref APP_REPOS_PATH: String = env::var("APP_REPOS_PATH")
        .unwrap_or_else(|_| panic!("APP_REPOS_PATH environment variable is not set"));
}

pub fn list_installed_app_repos() -> Vec<AppRepo> {
    list_installed_app_repo_paths()
        .into_iter()
        .filter_map(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| AppRepo {
                    name: name.to_string(),
                    apps: vec![],
                })
        })
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

pub fn app_repos_path() -> PathBuf {
    PathBuf::from(&*APP_REPOS_PATH)
}
