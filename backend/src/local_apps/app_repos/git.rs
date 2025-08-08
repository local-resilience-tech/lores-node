use anyhow::Error;
use git2::Repository;

use crate::local_apps::app_repos::AppRepoReference;

use super::{
    fs::{app_repo_at_source, app_repo_path},
    AppRepo, AppRepoSource,
};

#[derive(Debug)]
pub enum CreateRepoError {
    InvalidName,
    CloneFailed,
}

pub async fn clone_git_app_repo(repo: &AppRepoSource) -> Result<AppRepo, CreateRepoError> {
    let git_url = repo.git_url.clone();
    let name = repo.name.clone();

    if name.contains('/') || name.contains('\\') || name.is_empty() {
        return Err(CreateRepoError::InvalidName);
    }

    let repo_ref = AppRepoReference {
        repo_name: name.clone(),
    };
    let into = app_repo_path(&repo_ref);

    let git_repo = Repository::clone(&git_url, &into).map_err(|_| CreateRepoError::CloneFailed)?;

    println!("Cloned repository: {}", git_repo.path().display());

    match app_repo_at_source(&repo) {
        Some(app_repo) => Ok(app_repo),
        None => {
            eprintln!("Failed to find app cloned repository: {}", name);
            Err(CreateRepoError::CloneFailed)
        }
    }
}

pub fn git_origin_url(repo: &AppRepoReference) -> Result<String, Error> {
    let path = app_repo_path(repo);
    let git_repo = Repository::open(&path)?;

    let remote = git_repo.find_remote("origin")?;
    let url = remote.url().map(String::from);

    match url {
        Some(url) => Ok(url),
        None => Err(Error::msg("No origin URL found for the repository.")),
    }
}
