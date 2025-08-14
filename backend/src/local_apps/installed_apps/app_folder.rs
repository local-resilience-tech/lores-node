use std::{env, path::PathBuf};

use super::AppReference;

lazy_static! {
    pub static ref APPS_PATH: String =
        env::var("APPS_PATH").unwrap_or_else(|_| "../apps".to_string());
}

pub fn apps_path() -> PathBuf {
    PathBuf::from(APPS_PATH.clone())
}

pub struct AppFolder {
    #[allow(dead_code)]
    pub app_ref: AppReference,
    pub root_path: PathBuf,
}

impl AppFolder {
    pub fn new(app_ref: AppReference) -> Self {
        AppFolder {
            app_ref: app_ref.clone(),
            root_path: app_path(&app_ref),
        }
    }

    pub fn compose_file_path(&self) -> PathBuf {
        self.root_path.join("compose.yml")
    }

    pub fn copy_in_version(&self, source: &PathBuf) -> Result<(), ()> {
        copy_app_files(source, &self.root_path).map_err(|_| ())
    }
}

fn app_path(app_ref: &AppReference) -> PathBuf {
    apps_path().join(&app_ref.app_name)
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
