use git2::Repository;

pub fn fetch_origin_main(repo: &Repository) -> Result<(), git2::Error> {
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.download_tags(git2::AutotagOption::All);
    repo.find_remote("origin")?
        .fetch(&[] as &[&str], Some(&mut fetch_options), None)?;

    Ok(())
}
