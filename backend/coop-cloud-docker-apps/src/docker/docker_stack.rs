use std::process::Command;

use super::{DockerStack, helpers::parse_docker_json};

#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
struct DockerStackLsResult {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Services")]
    pub services: String,
}

pub fn docker_stack_ls() -> Result<Vec<DockerStack>, anyhow::Error> {
    let output = Command::new("docker")
        .arg("stack")
        .arg("ls")
        .arg("--format")
        .arg("json")
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute command: {}", e))?;

    let results = parse_docker_json::<Vec<DockerStackLsResult>>(output)?;

    let stacks: Vec<DockerStack> = results
        .into_iter()
        .map(|result| DockerStack {
            name: result.name,
            services_count: result.services.parse().unwrap_or(0),
        })
        .collect();

    Ok(stacks)
}

#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
pub struct DockerStackServicesResult {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Image")]
    pub image: String,

    #[serde(rename = "Mode")]
    pub mode: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Ports")]
    pub ports: String,
}

pub fn docker_stack_services(
    stack_name: &str,
) -> Result<Vec<DockerStackServicesResult>, anyhow::Error> {
    let output = Command::new("docker")
        .arg("stack")
        .arg("services")
        .arg(stack_name)
        .arg("--format")
        .arg("json")
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute command: {}", e))?;

    let services = parse_docker_json::<Vec<DockerStackServicesResult>>(output)?;

    Ok(services)
}
