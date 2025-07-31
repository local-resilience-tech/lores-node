use git2::Repository;

use super::{installed::app_repos_path, AppRepoSource};

#[derive(Debug)]
pub enum CreateRepoError {
    InvalidName,
}

pub async fn clone_git_app_repo(repo: &AppRepoSource) -> Result<(), CreateRepoError> {
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
