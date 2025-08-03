use super::{config::app_config_from_string, AppDefinition};
use std::path::PathBuf;

pub fn app_definition_at_path(path: &PathBuf) -> Option<AppDefinition> {
    if !path.exists() {
        eprintln!("App definition path does not exist: {}", path.display());
        return None;
    }

    let config_file_path = path.join("config/app.toml");
    match std::fs::read_to_string(config_file_path.clone()) {
        Ok(file_contents) => app_config_from_string(file_contents).ok(),
        Err(_) => {
            eprintln!("Could not read file `{}`", config_file_path.display());
            None
        }
    }
}

pub fn app_definitions_in_path(path: PathBuf) -> Vec<AppDefinition> {
    let app_paths = app_paths_in_path(&path);
    app_paths
        .into_iter()
        .filter_map(|app_path| app_definition_at_path(&app_path))
        .collect()
}

pub fn app_paths_in_path(path: &PathBuf) -> Vec<PathBuf> {
    if !path.exists() {
        eprintln!("App repo path does not exist: {}", path.display());
        return vec![];
    }

    println!("Finding apps in path: {}", path.display());

    std::fs::read_dir(path)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|p| p.is_dir())
        .filter(|p| p.file_name().and_then(|n| n.to_str()) != Some(".git"))
        .collect()
}
