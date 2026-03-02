use std::process::Command;

use crate::docker::helpers::parse_docker_json;

use super::{DockerService, DockerStack};

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

#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
pub struct DockerStackPsResult {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Image")]
    pub image: String,

    #[serde(rename = "Node")]
    pub node: String,

    #[serde(rename = "DesiredState")]
    pub desired_state: String,

    #[serde(rename = "CurrentState")]
    pub current_state: String,

    #[serde(rename = "Error")]
    pub error: Option<String>,

    #[serde(rename = "Ports")]
    pub ports: String,
}

pub fn docker_stack_ps(stack_name: &str) -> Result<Vec<DockerService>, anyhow::Error> {
    let output = Command::new("docker")
        .arg("stack")
        .arg("ps")
        .arg(stack_name)
        .arg("--format")
        .arg("json")
        .arg("--filter")
        .arg("desired-state=Running")
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute command: {}", e))?;

    let services = parse_docker_json::<Vec<DockerStackPsResult>>(output)?;

    let services: Vec<DockerService> = services
        .into_iter()
        .map(|result| {
            let (current_state, current_state_duration) =
                split_state_and_duration(&result.current_state);
            DockerService {
                id: result.id,
                name: result.name,
                image: result.image,
                node_name: result.node,
                current_state,
                current_state_duration,
            }
        })
        .collect();

    Ok(services)
}

fn split_state_and_duration(state: &str) -> (String, String) {
    let parts: Vec<&str> = state.splitn(2, ' ').collect();
    if parts.len() == 2 {
        (parts[0].to_string(), parts[1].to_string())
    } else {
        (state.to_string(), String::new())
    }
}
