use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AppVersionDefinition {
    pub name: String,
    pub version: String,
}

pub fn parse_app_definition(file_contents: String) -> Result<AppVersionDefinition, anyhow::Error> {
    let file_contents = file_contents.trim();
    serde_yaml::from_str::<AppVersionDefinition>(&file_contents).map_err(|e| {
        eprintln!("Could not parse YAML: {}", e);
        anyhow::anyhow!("Failed to parse YAML")
    })
}
