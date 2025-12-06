use git2::Repository;

pub fn checkout_latest_main(repo: &Repository) -> Result<(), git2::Error> {
    // First fetch the latest changes from origin
    fetch_origin_main(repo)?;

    // Find the remote tracking branch (origin/main)
    let remote_main_branch = repo.find_branch("origin/main", git2::BranchType::Remote)?;
    let remote_main_commit = remote_main_branch.get().peel_to_commit()?;

    // Update local main branch to point to origin/main
    let mut local_main_branch = repo.find_branch("main", git2::BranchType::Local)?;
    local_main_branch
        .get_mut()
        .set_target(remote_main_commit.id(), "Fast-forward main to origin/main")?;

    // Checkout the updated main branch
    repo.checkout_tree(remote_main_commit.as_object(), None)?;
    repo.set_head("refs/heads/main")?;

    Ok(())
}

pub fn fetch_origin_main(repo: &Repository) -> Result<(), git2::Error> {
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.download_tags(git2::AutotagOption::All);
    repo.find_remote("origin")?
        .fetch(&[] as &[&str], Some(&mut fetch_options), None)?;

    Ok(())
}
