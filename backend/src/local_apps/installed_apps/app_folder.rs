use std::path::PathBuf;

use super::{
    super::shared::app_definitions::{parse_app_definition, AppVersionDefinition},
    apps_folder::AppsFolder,
    AppReference,
};

pub struct InstalledAppDetails {
    pub name: String,
    pub version: String,
}

pub struct AppFolder {
    pub app_ref: AppReference,
    pub apps_folder: AppsFolder,
}

impl AppFolder {
    pub fn new(app_ref: AppReference) -> Self {
        AppFolder {
            app_ref: app_ref.clone(),
            apps_folder: AppsFolder::new(),
        }
    }

    pub fn app_definition_file_path(&self) -> PathBuf {
        self.current_version_path().join("lores_app.yml")
    }

    pub fn app_details(&self) -> Option<InstalledAppDetails> {
        let config_file_path = self.app_definition_file_path();
        match std::fs::read_to_string(config_file_path.clone()) {
            Ok(file_contents) => {
                parse_app_definition(file_contents)
                    .ok()
                    .map(|def: AppVersionDefinition| InstalledAppDetails {
                        name: def.name,
                        version: def.version,
                    })
            }
            Err(_) => {
                eprintln!("Could not read file `{}`", config_file_path.display());
                None
            }
        }
    }

    fn current_version_path(&self) -> PathBuf {
        self.root_path(None).join("current")
    }

    fn root_path(&self, override_base: Option<String>) -> PathBuf {
        let base_path = match override_base {
            Some(path) => PathBuf::from(path),
            None => self.apps_folder.path(),
        };
        base_path.join(&self.app_ref.app_name)
    }
}
