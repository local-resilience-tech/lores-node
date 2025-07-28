use git2::Repository;
use std::{env, path::PathBuf};

use super::AppRepo;

lazy_static! {
    pub static ref APP_REPOS_PATH: String = env::var("APP_REPOS_PATH")
        .unwrap_or_else(|_| panic!("APP_REPOS_PATH environment variable is not set"));
}

#[derive(Debug)]
pub enum CreateRepoError {
    InvalidName,
}

pub async fn clone_git_app_repo(repo: &AppRepo) -> Result<(), CreateRepoError> {
    let git_url = repo.git_url.clone();
    let name = repo.name.clone();

    if name.contains('/') || name.contains('\\') || name.is_empty() {
        return Err(CreateRepoError::InvalidName);
    }

    let into = app_repos_path().join(name);

    let git_repo = match Repository::clone(&git_url, &into) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    println!("Cloned repository: {}", git_repo.path().display());
    Ok(())
}

fn app_repos_path() -> PathBuf {
    PathBuf::from(&*APP_REPOS_PATH)
}
