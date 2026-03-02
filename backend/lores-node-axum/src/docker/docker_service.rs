use std::{collections::HashMap, process::Command};

use crate::docker::helpers::parse_docker_json;

#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
pub struct DockerInspectSpec {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    #[serde(rename = "TaskTemplate")]
    pub task_template: serde_json::Value,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
pub struct DockerInspectResult {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Version")]
    pub version: serde_json::Value,

    #[serde(rename = "CreatedAt")]
    pub created_at: String,

    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,

    #[serde(rename = "Spec")]
    pub spec: DockerInspectSpec,
}

pub fn docker_service_inspect(name: &str) -> Result<DockerInspectResult, anyhow::Error> {
    let output = Command::new("docker")
        .arg("service")
        .arg("inspect")
        .arg(name)
        .arg("--format")
        .arg("json")
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute command: {}", e))?;

    let details = parse_docker_json::<Vec<DockerInspectResult>>(output)?;

    // Since docker inspect returns an array, we take the first element
    let detail = details
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No details found for service: {}", name))?;

    Ok(detail)
}
