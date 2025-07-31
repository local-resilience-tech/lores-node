use git2::Repository;

use super::{
    installed::{app_repo_at_reference, app_repos_path},
    AppRepo, AppRepoReference, AppRepoSource,
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

    let into = app_repos_path().join(name.clone());

    let git_repo = Repository::clone(&git_url, &into).map_err(|_| CreateRepoError::CloneFailed)?;

    println!("Cloned repository: {}", git_repo.path().display());

    let repo_ref = AppRepoReference { name: name.clone() };

    match app_repo_at_reference(&repo_ref) {
        Some(app_repo) => Ok(app_repo),
        None => {
            eprintln!("Failed to find app cloned repository: {}", name);
            Err(CreateRepoError::CloneFailed)
        }
    }
}
