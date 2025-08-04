use std::process::Command;

use super::DockerStack;

#[derive(Debug, Clone, serde::Deserialize)]
struct DockerStackLsResult {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Services")]
    services: String,
}

pub fn docker_stack_ls() -> Result<Vec<DockerStack>, anyhow::Error> {
    let output = Command::new("docker")
        .arg("stack")
        .arg("ls")
        .arg("--format")
        .arg("json")
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute command: {}", e))?;

    let stdout_string = String::from_utf8(output.stdout)
        .map_err(|e| anyhow::anyhow!("Failed to convert output to string: {}", e))?;
    let stdout_string = ensure_json_is_array(&stdout_string);

    println!("Docker stack ls output: {}", stdout_string);

    let results = serde_json::from_str::<Vec<DockerStackLsResult>>(&stdout_string)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;

    let stacks: Vec<DockerStack> = results
        .into_iter()
        .map(|result| DockerStack {
            name: result.name,
            services_count: result.services.parse().unwrap_or(0),
        })
        .collect();

    Ok(stacks)
}

fn ensure_json_is_array(input: &str) -> String {
    if input.starts_with('[') && input.ends_with(']') {
        input.to_string()
    } else {
        format!("[{}]", input)
    }
}
