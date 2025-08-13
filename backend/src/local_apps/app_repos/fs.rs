use semver::Version;
use std::{env, path::PathBuf};

use super::{
    super::shared::app_definitions::AppVersionDefinition,
    git_app_repos::{git_origin_url, git_version_tags},
    AppDefinition, AppRepo, AppRepoAppReference, AppRepoReference, AppRepoSource,
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

pub fn app_repo_from_app_name(app_name: &str) -> Option<AppRepoReference> {
    list_installed_app_repos()
        .into_iter()
        .find(|repo| repo.apps.iter().any(|app| app.name == app_name))
        .map(|repo| AppRepoReference {
            repo_name: repo.name,
        })
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
    let apps = versioned_app_definitions_in_repo(&repo_ref);

    Some(AppRepo {
        name: repo_src.name.clone(),
        git_url: repo_src.git_url.clone(),
        apps,
    })
}

pub fn app_repo_from_ref(repo_ref: &AppRepoReference) -> Result<AppRepo, anyhow::Error> {
    let apps = versioned_app_definitions_in_repo(&repo_ref);
    let git_url = git_origin_url(repo_ref).map_err(|e| {
        anyhow::Error::msg(format!("Failed to get git URL for app repository: {}", e))
    })?;

    Ok(AppRepo {
        name: repo_ref.repo_name.clone(),
        git_url,
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

fn versioned_app_definitions_in_repo(repo_ref: &AppRepoReference) -> Vec<AppDefinition> {
    let tag_versions = git_version_tags(repo_ref).unwrap_or_default();
    combine_app_definitions(tag_versions)
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

pub fn combine_app_definitions(defs: Vec<AppVersionDefinition>) -> Vec<AppDefinition> {
    let mut combined: Vec<AppDefinition> = Vec::new();

    for def in defs {
        if let Some(existing) = combined.iter_mut().find(|d| d.name == def.name) {
            if !existing.versions.contains(&def.version) {
                existing.versions.push(def.version);
            }
        } else {
            combined.push(AppDefinition {
                name: def.name,
                versions: vec![def.version.clone()],
                latest_version: None,
            });
        }
    }

    for app_def in &mut combined {
        app_def.versions = sort_versions_latest_first(&app_def.versions);
        app_def.latest_version = app_def.versions.first().cloned();
    }

    combined
}

fn sort_versions_latest_first(input_versions: &Vec<String>) -> Vec<String> {
    let mut versions = input_versions
        .iter()
        .filter_map(|v| {
            // Only include valid semver versions
            Version::parse(&v).ok().map(|parsed| (v, parsed))
        })
        .collect::<Vec<_>>();

    versions.sort_by(|a, b| b.1.cmp(&a.1)); // descending order

    versions.into_iter().map(|(v, _)| v.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_app_definitions_empty() {
        let defs: Vec<AppVersionDefinition> = vec![];
        let combined = combine_app_definitions(defs);
        assert!(combined.is_empty());
    }

    #[test]
    fn test_combine_app_definitions_single() {
        let defs = vec![AppVersionDefinition {
            name: "app1".to_string(),
            version: "1.0.0".to_string(),
            // add other fields as needed
        }];
        let combined = combine_app_definitions(defs);
        assert_eq!(combined.len(), 1);
        assert_eq!(combined[0].name, "app1");
        assert_eq!(combined[0].versions, vec!["1.0.0"]);
    }

    #[test]
    fn test_combine_app_definitions_multiple_versions() {
        let defs = vec![
            AppVersionDefinition {
                name: "app1".to_string(),
                version: "1.0.0".to_string(),
            },
            AppVersionDefinition {
                name: "app1".to_string(),
                version: "1.1.0".to_string(),
            },
            AppVersionDefinition {
                name: "app2".to_string(),
                version: "2.0.0".to_string(),
            },
        ];
        let mut combined = combine_app_definitions(defs);
        combined.sort_by(|a, b| a.name.cmp(&b.name));
        assert_eq!(combined.len(), 2);

        assert_eq!(combined[0].name, "app1");
        assert_eq!(combined[0].versions, vec!["1.1.0", "1.0.0"]);
        assert_eq!(combined[0].latest_version, Some("1.1.0".to_string()));

        assert_eq!(combined[1].name, "app2");
        assert_eq!(combined[1].versions, vec!["2.0.0"]);
        assert_eq!(combined[1].latest_version, Some("2.0.0".to_string()));
    }

    #[test]
    fn test_combine_app_definitions_removes_duplicates() {
        let defs = vec![
            AppVersionDefinition {
                name: "app1".to_string(),
                version: "1.0.0".to_string(),
            },
            AppVersionDefinition {
                name: "app1".to_string(),
                version: "1.0.0".to_string(),
            },
        ];
        let combined = combine_app_definitions(defs);
        assert_eq!(combined.len(), 1);
        assert_eq!(combined[0].name, "app1");
        assert_eq!(combined[0].versions, vec!["1.0.0"]);
    }

    #[test]
    fn test_sort_versions_basic() {
        let input = vec![
            "1.0.0".to_string(),
            "2.0.0".to_string(),
            "0.9.0".to_string(),
        ];
        let sorted = sort_versions_latest_first(&input);
        assert_eq!(sorted, vec!["2.0.0", "1.0.0", "0.9.0"]);
    }

    #[test]
    fn test_sort_versions_with_invalid_versions() {
        let input = vec![
            "1.0.0".to_string(),
            "not-a-version".to_string(),
            "2.0.0".to_string(),
        ];
        let sorted = sort_versions_latest_first(&input);
        // "not-a-version" should be removed
        assert_eq!(sorted, vec!["2.0.0", "1.0.0"]);
    }

    #[test]
    fn test_sort_versions_all_invalid() {
        let input = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];
        let sorted = sort_versions_latest_first(&input);
        // All invalid, so result should be empty
        assert!(sorted.is_empty());
    }

    #[test]
    fn test_sort_versions_with_prerelease() {
        let input = vec![
            "1.0.0-alpha".to_string(),
            "1.0.0".to_string(),
            "1.0.1-beta".to_string(),
            "1.0.1".to_string(),
        ];
        let sorted = sort_versions_latest_first(&input);
        // 1.0.1 > 1.0.1-beta > 1.0.0 > 1.0.0-alpha
        assert_eq!(sorted, vec!["1.0.1", "1.0.1-beta", "1.0.0", "1.0.0-alpha"]);
    }

    #[test]
    fn test_sort_versions_duplicates() {
        let input = vec![
            "1.0.0".to_string(),
            "1.0.0".to_string(),
            "2.0.0".to_string(),
        ];
        let sorted = sort_versions_latest_first(&input);
        assert_eq!(sorted, vec!["2.0.0", "1.0.0", "1.0.0"]);
    }

    #[test]
    fn test_sort_versions_empty() {
        let input: Vec<String> = vec![];
        let sorted = sort_versions_latest_first(&input);
        assert!(sorted.is_empty());
    }
}
