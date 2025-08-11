use super::AppVersionDefinition;

pub fn app_config_from_string(file_contents: String) -> Result<AppVersionDefinition, anyhow::Error> {
    match toml::from_str::<AppVersionDefinition>(&file_contents) {
        Ok(contents) => Ok(contents),
        Err(e) => {
            eprintln!("Could not parse TOML: {}", e);
            Err(anyhow::anyhow!("Failed to parse TOML"))
        }
    }
}
