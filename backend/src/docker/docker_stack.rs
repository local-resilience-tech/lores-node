use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use super::{DockerService, DockerStack};

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
) -> Result<(), anyhow::Error> {
    if compose_files.is_empty() {
        return Err(anyhow::anyhow!(
            "At least one compose file must be provided"
        ));
    }

    // First get the composed config
    let mut config_command = docker_compose_output_config_command(compose_files);
    config_command.stdout(Stdio::piped());

    println!(
        "Running compose config command: {:?} {:?}",
        config_command.get_program(),
        config_command.get_args()
    );

    let config_child = config_command
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to start compose config command: {}", e))?;
    let config_out = config_child.stdout.expect("Failed to open config stdout");

    // Create a sed command that reads from config output
    let mut sed_command = Command::new("sed");
    sed_command
        .arg("-e")
        .arg("/published:/ s/\"//g")
        .arg("-e")
        .arg("/^name\\:/d")
        .stdin(Stdio::from(config_out))
        .stdout(Stdio::piped());

    let sed_child = sed_command
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to start sed command: {}", e))?;
    let sed_out = sed_child.stdout.expect("Failed to open sed stdout");

    // Create a deploy command that reads from sed output
    let mut deploy_command = Command::new("docker");
    deploy_command
        .arg("stack")
        .arg("deploy")
        .arg("--compose-file")
        .arg("-") // Read from stdin
        .arg(stack_name);

    // Run the deploy command with the sed output as stdin
    let deploy_child = deploy_command
        .stdin(Stdio::from(sed_out))
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to start deploy command: {}", e))?;

    let output = deploy_child
        .wait_with_output()
        .expect("Failed to wait on deploy");

    println!("Successfully deployed stack: {}", stack_name);
    println!("Deploy output: {}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn docker_compose_output_config_command(compose_files: &[PathBuf]) -> Command {
    let mut command = Command::new("docker");
    command.arg("compose");

    // Add each compose file with its own -f argument
    for file in compose_files {
        command.arg("-f").arg(file);
    }

    command.arg("config");

    command.arg("--format").arg("yaml");

    command
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
