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
