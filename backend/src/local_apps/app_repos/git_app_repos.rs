use std::path::PathBuf;

use anyhow::Error;
use git2::Repository;
use semver::VersionReq;

use crate::local_apps::{
    app_repos::{AppRepoAppReference, AppRepoReference},
    shared::app_definitions::AppVersionDefinition,
};

use super::{
    fs::{app_repo_app_path, app_repo_at_source, app_repo_path},
    git_commands, AppRepo, AppRepoSource,
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
    let git_repo = open_repository(repo)?;

    let remote = git_repo.find_remote("origin")?;
    let url = remote.url().map(String::from);

    match url {
        Some(url) => Ok(url),
        None => Err(Error::msg("No origin URL found for the repository.")),
    }
}

pub fn git_version_tags(repo: &AppRepoReference) -> Result<Vec<AppVersionDefinition>, Error> {
    fetch_origin_main(repo)?;

    let git_repo = open_repository(repo)?;

    let tags = git_repo.tag_names(None)?;

    let definitions: Vec<AppVersionDefinition> = tags
        .into_iter()
        .filter(|tag| tag.is_some())
        .filter_map(|tag| {
            let tag = tag?;
            tag_to_app_definition(tag)
        })
        .collect::<Vec<AppVersionDefinition>>()
        .into();

    Ok(definitions)
}

#[derive(Debug)]
pub enum CheckoutAppVersionError {
    InUse,
    Other(Error),
    CallbackError(Error),
}

impl CheckoutAppVersionError {
    #[allow(dead_code)]
    pub fn as_inner(&self) -> Option<&Error> {
        match self {
            CheckoutAppVersionError::Other(err) => Some(err),
            _ => None,
        }
    }
}

pub fn with_checked_out_app_version(
    app_ref: &AppRepoAppReference,
    callback: impl FnOnce(&PathBuf) -> Result<(), CheckoutAppVersionError>,
) -> Result<(), CheckoutAppVersionError> {
    let repo_ref = app_ref.repo_ref();

    let in_use = is_detached(&repo_ref).map_err(|e| CheckoutAppVersionError::Other(e))?;

    if in_use {
        println!(
            "Cannot checkout app version {} for repo {} because it is already detached, that probably means another process is using it.",
            app_ref.version, app_ref.repo_name,
        );
        return Err(CheckoutAppVersionError::InUse);
    }
    checkout_app_version(&app_ref).map_err(|e| CheckoutAppVersionError::Other(e))?;

    let path = app_repo_app_path(&app_ref);
    let callback_result = callback(&path);

    checkout_latest_main(&repo_ref).map_err(|e| CheckoutAppVersionError::Other(e))?;

    callback_result?;

    Ok(())
}

fn is_detached(repo_ref: &AppRepoReference) -> Result<bool, Error> {
    let repo = open_repository(repo_ref)?;
    repo.head_detached().map_err(Error::from)
}

pub fn checkout_app_version(app_ref: &AppRepoAppReference) -> Result<(), Error> {
    let repo_ref = app_ref.repo_ref();
    fetch_origin_main(&repo_ref)?;

    let repo = open_repository(&repo_ref)?;
    let tag_name = app_ref_to_git_tag(app_ref);

    git_commands::checkout_tag_detatched(&repo, &tag_name)?;

    println!(
        "Checked out tag: {} on repo {}",
        tag_name, repo_ref.repo_name
    );

    Ok(())
}

pub fn checkout_latest_main(repo_ref: &AppRepoReference) -> Result<(), Error> {
    let git_repo = open_repository(repo_ref)?;

    git_commands::checkout_latest_main(&git_repo)?;

    println!(
        "Checked out latest main branch from origin for: {}",
        repo_ref.repo_name
    );

    Ok(())
}

fn fetch_origin_main(repo_ref: &AppRepoReference) -> Result<(), Error> {
    let git_repo = open_repository(repo_ref)?;
    git_commands::fetch_origin_main(&git_repo).map_err(Error::from)
}

pub fn tag_to_app_definition(tag: &str) -> Option<AppVersionDefinition> {
    // Expect format: "name-vX.Y.Z"
    let dash_pos = tag.rfind("-v")?;
    let (name, version_part) = tag.split_at(dash_pos);
    let version = &version_part[2..]; // skip "-v"
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() == 3 && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit())) {
        // Check if version is valid semver
        if VersionReq::parse(version).is_err() {
            eprintln!("Invalid semver for AppDefinition: {}", tag);
            return None;
        }
        Some(AppVersionDefinition {
            name: name.to_string(),
            version: version.to_string(),
        })
    } else {
        eprintln!("Invalid tag for AppDefinition: {}", tag);
        None
    }
}

fn app_ref_to_git_tag(app_ref: &AppRepoAppReference) -> String {
    format!("{}-v{}", app_ref.app_name, app_ref.version)
}

fn open_repository(repo_ref: &AppRepoReference) -> Result<Repository, Error> {
    let path = app_repo_path(repo_ref);
    Repository::open(&path).map_err(Error::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_to_app_definition_valid_tag() {
        let tag = "myapp-v1.2.3";
        let def = tag_to_app_definition(tag);
        assert!(def.is_some());
        let def = def.unwrap();
        assert_eq!(def.name, "myapp");
        assert_eq!(def.version, "1.2.3");
    }

    #[test]
    fn test_tag_to_app_definition_invalid_tag_no_version() {
        let tag = "myapp";
        let def = tag_to_app_definition(tag);
        assert!(def.is_none());
    }

    #[test]
    fn test_tag_to_app_definition_invalid_tag_bad_version() {
        let tag = "myapp-v1.2";
        let def = tag_to_app_definition(tag);
        assert!(def.is_none());
    }

    #[test]
    fn test_tag_to_app_definition_invalid_tag_extra_dash() {
        let tag = "my-app-v1.2.3";
        let def = tag_to_app_definition(tag);
        assert!(def.is_some());
        let def = def.unwrap();
        assert_eq!(def.name, "my-app");
        assert_eq!(def.version, "1.2.3");
    }

    #[test]
    fn test_tag_to_app_definition_invalid_tag_non_numeric_version() {
        let tag = "myapp-vx.y.z";
        let def = tag_to_app_definition(tag);
        assert!(def.is_none());
    }

    #[test]
    fn test_tag_to_app_definition_valid_tag_with_zeros() {
        let tag = "foo-v0.0.0";
        let def = tag_to_app_definition(tag);
        assert!(def.is_some());
        let def = def.unwrap();
        assert_eq!(def.name, "foo");
        assert_eq!(def.version, "0.0.0");
    }

    #[test]
    fn test_tag_to_app_definition_invalid_semver() {
        // This is not a valid semver version requirement
        let tag = "bar-v1.2.3.4";
        let def = tag_to_app_definition(tag);
        assert!(def.is_none());

        let tag = "baz-v1.2.3-beta";
        let def = tag_to_app_definition(tag);
        // This will fail because parts.len() != 3, so it's not accepted
        assert!(def.is_none());

        let tag = "baz-v1.2.3";
        let def = tag_to_app_definition(tag);
        assert!(def.is_some());
    }

    #[test]
    fn test_tag_to_app_definition_valid_semver_variants() {
        // Only X.Y.Z is accepted, so pre-release and build metadata are not
        let tag = "myapp-v1.2.3";
        let def = tag_to_app_definition(tag);
        assert!(def.is_some());

        let tag = "myapp-v1.2.3-alpha";
        let def = tag_to_app_definition(tag);
        assert!(def.is_none());

        let tag = "myapp-v1.2.3+build";
        let def = tag_to_app_definition(tag);
        assert!(def.is_none());
    }
}
