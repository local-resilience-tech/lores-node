use std::{
    collections::HashMap,
    path::PathBuf,
    process::{Command, Stdio},
};

use super::{docker_compose::docker_compose_app_file, DockerService, DockerStack};
use std::io::Write;

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
    let stdout_string = json_object_lines_to_array(&stdout_string);

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

#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
struct DockerStackPsResult {
    #[serde(rename = "ID")]
    id: String,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Image")]
    image: String,

    #[serde(rename = "Node")]
    node: String,

    #[serde(rename = "DesiredState")]
    desired_state: String,

    #[serde(rename = "CurrentState")]
    current_state: String,

    #[serde(rename = "Error")]
    error: Option<String>,

    #[serde(rename = "Ports")]
    ports: String,
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

    let stdout_string = String::from_utf8(output.stdout)
        .map_err(|e| anyhow::anyhow!("Failed to convert output to string: {}", e))?;
    let stdout_string = json_object_lines_to_array(&stdout_string);

    println!("Docker stack ps output: {}", stdout_string);

    let services = serde_json::from_str::<Vec<DockerStackPsResult>>(&stdout_string)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;

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

pub fn docker_stack_rm(stack_name: &str) -> Result<(), anyhow::Error> {
    let output = Command::new("docker")
        .arg("stack")
        .arg("rm")
        .arg(stack_name)
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!(
            "Failed to remove stack '{}': {}",
            stack_name,
            stderr
        ));
    }

    println!("Successfully removed stack: {}", stack_name);
    Ok(())
}

pub fn docker_stack_compose_and_deploy(
    stack_name: &str,
    compose_files: &[PathBuf],
    compose_env_vars: &HashMap<String, String>,
    deploy_env_vars: &HashMap<String, String>,
) -> Result<(), anyhow::Error> {
    let processed_config = docker_compose_app_file(compose_files, compose_env_vars)?;

    // Print the processed config for debugging
    println!("Processed config for deployment:\n{}", processed_config);

    docker_stack_deploy(stack_name, &processed_config, deploy_env_vars)
}

pub fn docker_stack_deploy(
    stack_name: &str,
    input_compose_contents: &str,
    deploy_env_vars: &HashMap<String, String>,
) -> Result<(), anyhow::Error> {
    // Create a deploy command that reads the processed config from stdin
    let mut deploy_command = Command::new("docker");
    deploy_command
        .arg("stack")
        .arg("deploy")
        .arg("--compose-file")
        .arg("-") // Read from stdin
        .envs(deploy_env_vars)
        .arg(stack_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Run the deploy command with the processed config as stdin
    let mut deploy_child = deploy_command
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to start deploy command: {}", e))?;

    // Take ownership of stdin and write to it
    if let Some(mut stdin) = deploy_child.stdin.take() {
        stdin
            .write_all(input_compose_contents.as_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to write to deploy command stdin: {}", e))?;
        // stdin is automatically dropped here when it goes out of scope
    }

    let output = deploy_child
        .wait_with_output()
        .expect("Failed to wait on deploy");

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Deploy failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    println!("Successfully deployed stack: {}", stack_name);
    println!("Deploy output: {}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn split_state_and_duration(state: &str) -> (String, String) {
    let parts: Vec<&str> = state.splitn(2, ' ').collect();
    if parts.len() == 2 {
        (parts[0].to_string(), parts[1].to_string())
    } else {
        (state.to_string(), String::new())
    }
}

fn json_object_lines_to_array(input: &str) -> String {
    let mut lines = input.lines().map(str::trim).filter(|line| !line.is_empty());
    let first_line = lines.next().unwrap_or("");
    let mut result = String::from("[");
    result.push_str(first_line);

    for line in lines {
        result.push_str(",");
        result.push_str(line);
    }

    result.push(']');
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_object_lines_to_array_empty() {
        let input = "";
        let expected = "[]";
        let result = json_object_lines_to_array(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_json_object_lines_to_array_single_line() {
        let input = r#"{"Name":"stack1","Services":"2"}"#;
        let expected = r#"[{"Name":"stack1","Services":"2"}]"#;
        let result = json_object_lines_to_array(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_json_object_lines_to_array_multiple_lines() {
        let input = r#"{"Name":"stack1","Services":"2"}
{"Name":"stack2","Services":"3"}"#;
        let expected = r#"[{"Name":"stack1","Services":"2"},{"Name":"stack2","Services":"3"}]"#;
        let result = json_object_lines_to_array(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_json_object_lines_to_array_trailing_newline() {
        let input = r#"{"Name":"stack1","Services":"2"}
{"Name":"stack2","Services":"3"}
"#;
        let expected = r#"[{"Name":"stack1","Services":"2"},{"Name":"stack2","Services":"3"}]"#;
        let result = json_object_lines_to_array(input);
        assert_eq!(result, expected);
    }
}
