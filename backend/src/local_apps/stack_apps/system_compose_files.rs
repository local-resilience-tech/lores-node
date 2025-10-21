use std::path::PathBuf;

pub struct SystemComposeFiles {
    path: PathBuf,
}

impl SystemComposeFiles {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn ensure_system_compose_files(&self) -> Result<(), anyhow::Error> {
        // Check if the directory exists
        if !self.path.exists() {
            return Err(anyhow::anyhow!(
                "System compose files path does not exist: {}",
                self.path.display()
            ));
        }

        // Make sure it's a directory
        if !self.path.is_dir() {
            return Err(anyhow::anyhow!(
                "System compose files path is not a directory: {}",
                self.path.display()
            ));
        }

        self.ensure_reset_file()?;
        self.ensure_setup_file()?;

        Ok(())
    }

    pub fn reset_path(&self) -> PathBuf {
        self.path.join("system-compose-reset-2025-10-08.yml")
    }

    pub fn setup_path(&self) -> PathBuf {
        self.path.join("system-compose-setup-2025-10-08.yml")
    }

    fn ensure_reset_file(&self) -> Result<(), anyhow::Error> {
        let file_path = self.reset_path();
        let content = r#"version: "3.8"
volumes: !reset null
networks: !reset null
"#;

        self.ensure_system_compose_file(file_path, content, true)
    }

    fn ensure_setup_file(&self) -> Result<(), anyhow::Error> {
        let file_path = self.setup_path();
        let content = r#"version: "3.8"
volumes:
  lores_config:
    driver: local
    driver_opts:
      type: 'none'
      o: 'bind'
      device: '${HOST_OS_APP_CONFIG_DIR}'
    name: 'lores_app_config_${LORES_APP_NAME}'
networks:
  lores:
    external: true
"#;

        self.ensure_system_compose_file(file_path, content, true)
    }

    fn ensure_system_compose_file(
        &self,
        file_path: PathBuf,
        content: &str,
        force_overwrite: bool,
    ) -> Result<(), anyhow::Error> {
        // Skip if the file already exists and we're not forcing an overwrite
        if file_path.exists() && !force_overwrite {
            return Ok(());
        }

        // Write the file to disk
        std::fs::write(&file_path, content).map_err(|e| {
            anyhow::anyhow!("Failed to write compose file {:?}: {:?}", file_path, e)
        })?;

        println!("Created system compose file at: {}", file_path.display());

        Ok(())
    }
}
