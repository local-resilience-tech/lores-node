use super::{
    fs::{load_config_schema_text, load_config_text},
    AppReference,
};

pub fn load_app_config_schema(
    app_ref: &AppReference,
) -> Result<Option<serde_json::Value>, anyhow::Error> {
    let schema_text = load_config_schema_text(app_ref)?;
    let schema_json = serde_json::from_str(&schema_text)?;

    // Validate schema
    if !jsonschema::meta::is_valid(&schema_json) {
        anyhow::bail!("Schema is not a valid JSON Schema");
    }
    jsonschema::meta::validate(&schema_json)
        .map_err(|e| anyhow::anyhow!("Schema validation failed: {}", e))?;

    Ok(Some(schema_json))
}

pub fn load_app_config(app_ref: &AppReference) -> Result<serde_json::Value, anyhow::Error> {
    let config_text = load_config_text(app_ref)?;
    let config_json = serde_json::from_str(&config_text)?;

    Ok(config_json)
}
