use semver::Version;
use std::{os::unix::fs::symlink, path::PathBuf};

use super::{
    super::shared::app_definitions::{parse_app_definition, AppVersionDefinition},
    apps_folder::AppsFolder,
    AppReference,
};

pub struct InstalledAppDetails {
    pub name: String,
    pub version: String,
    pub has_config_schema: bool,
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

    pub fn compose_file_path(&self) -> PathBuf {
        self.current_version_path().join("compose.yml")
    }

    pub fn merged_compose_file_path(&self) -> PathBuf {
        self.intermediate_dir_path().join("merged_compose.yml")
    }

    pub fn interpolated_compose_file_path(&self) -> PathBuf {
        self.intermediate_dir_path()
            .join("interpolated_compose.yml")
    }

    pub fn intermediate_dir_path(&self) -> PathBuf {
        self.root_path(None).join("intermediate")
    }

    pub fn app_definition_file_path(&self) -> PathBuf {
        self.current_version_path().join("lores_app.yml")
    }

    pub fn config_schema_file_path(&self) -> PathBuf {
        self.current_version_path().join("config_schema.json")
    }

    pub fn config_file_path(&self) -> PathBuf {
        self.config_dir_path(None).join("config.json")
    }

    pub fn config_dir_path(&self, override_base: Option<String>) -> PathBuf {
        self.app_data_path(override_base).join("lores_config")
    }

    pub fn app_data_path(&self, override_base: Option<String>) -> PathBuf {
        self.root_path(override_base).join("data")
    }

    pub fn has_config_schema(&self) -> bool {
        println!(
            "Checking if config schema exists at: {}",
            self.config_schema_file_path().display()
        );
        let exists = self.config_schema_file_path().exists();
        println!("Config schema exists: {}", exists);
        exists
    }

    pub fn ensure_exists(&self) -> Result<(), ()> {
        ensure_path(&self.root_path(None))?;
        ensure_path(&self.versions_path())?;
        ensure_path(&self.app_data_path(None))?;
        ensure_path(&self.config_dir_path(None))?;
        ensure_path(&self.intermediate_dir_path())?;

        Ok(())
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
                        has_config_schema: self.has_config_schema(),
                    })
            }
            Err(_) => {
                eprintln!("Could not read file `{}`", config_file_path.display());
                None
            }
        }
    }

    pub fn copy_in_version(&self, source: &PathBuf, version: &str) -> Result<(), ()> {
        if !version_valid(version) {
            eprintln!("Invalid version format: {}", version);
            return Err(());
        }

        let target = self.version_path(version);
        copy_app_files(source, &target).map_err(|_| ())
    }

    pub fn make_current_version(&self, version: &str) -> Result<(), ()> {
        let version_path = &self.version_path(version);
        let symlink_path = self.current_version_path();

        assert_path_exists(&version_path)
            .map_err(|_| eprintln!("Version path does not exist: {}", version_path.display()))?;

        remove_symlink_if_exists(&symlink_path).map_err(|_| {
            eprintln!(
                "Failed to remove existing symlink at `{}`",
                symlink_path.display()
            );
        })?;

        symlink(self.version_relative_path(version), &symlink_path).map_err(|_| {
            eprintln!(
                "Failed to create symlink from `{}` to `{}`",
                version_path.display(),
                symlink_path.display()
            );
            ()
        })?;

        Ok(())
    }

    fn current_version_path(&self) -> PathBuf {
        self.root_path(None).join("current")
    }

    fn version_path(&self, version: &str) -> PathBuf {
        self.versions_path().join(version)
    }

    fn version_relative_path(&self, version: &str) -> String {
        format!("versions/{}", version)
    }

    fn versions_path(&self) -> PathBuf {
        self.root_path(None).join("versions")
    }

    fn root_path(&self, override_base: Option<String>) -> PathBuf {
        let base_path = match override_base {
            Some(path) => PathBuf::from(path),
            None => self.apps_folder.path(),
        };
        base_path.join(&self.app_ref.app_name)
    }
}

fn copy_app_files(source: &PathBuf, target: &PathBuf) -> Result<(), ()> {
    match copy_dir::copy_dir(source, target) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!(
                "Failed to copy files from `{}` to `{}`: {}",
                source.display(),
                target.display(),
                e
            );
            Err(())
        }
    }
}

fn version_valid(version: &str) -> bool {
    // Simple version validation logic, can be expanded as needed
    !version.is_empty() && Version::parse(&version).is_ok()
}

fn ensure_path(path: &PathBuf) -> Result<(), ()> {
    if !path.exists() {
        std::fs::create_dir_all(path).map_err(|_| ())?;
    }
    Ok(())
}

fn assert_path_exists(path: &PathBuf) -> Result<(), ()> {
    match path.exists() {
        true => Ok(()),
        false => Err(()),
    }
}

fn remove_symlink_if_exists(path: &PathBuf) -> Result<(), ()> {
    if path.exists() {
        std::fs::remove_file(path).map_err(|_| ())?;
    }
    Ok(())
}
