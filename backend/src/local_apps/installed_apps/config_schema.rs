use super::{fs::load_config_schema_text, AppReference};

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

pub fn validate_app_config(
    app_ref: &AppReference,
    config: &serde_json::Value,
) -> Result<(), anyhow::Error> {
    let schema = match load_app_config_schema(app_ref)? {
        Some(s) => s,
        None => return Ok(()), // No schema means no validation needed
    };

    match validate_config_against_schema(&schema, config) {
        true => Ok(()),
        false => anyhow::bail!("Configuration does not conform to schema"),
    }
}

pub fn save_app_config(
    app_ref: &AppReference,
    config: &serde_json::Value,
) -> Result<(), anyhow::Error> {
    println!("Saving config for app: {:?}", app_ref);
    Ok(())
}

fn validate_config_against_schema(
    schema: &serde_json::Value,
    instance: &serde_json::Value,
) -> bool {
    jsonschema::is_valid(schema, instance)
}
