use std::{
    collections::HashMap,
    path::PathBuf,
    process::{Command, Stdio},
};

pub fn docker_compose_app_file(
    compose_files: &[PathBuf],
    compose_env_vars: &HashMap<String, String>,
) -> Result<String, anyhow::Error> {
    if compose_files.is_empty() {
        return Err(anyhow::anyhow!(
            "At least one compose file must be provided"
        ));
    }

    // First get the composed config
    let mut config_command = docker_compose_output_config_command(compose_files, compose_env_vars);
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

    // Capture sed output in a variable
    let sed_output = sed_command
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run sed command: {}", e))?;

    if !sed_output.status.success() {
        return Err(anyhow::anyhow!(
            "sed command failed: {}",
            String::from_utf8_lossy(&sed_output.stderr)
        ));
    }

    let processed_config = String::from_utf8(sed_output.stdout.clone())
        .map_err(|e| anyhow::anyhow!("Invalid UTF-8 in processed config: {}", e))?;

    Ok(processed_config)
}

fn docker_compose_output_config_command(
    compose_files: &[PathBuf],
    compose_env_vars: &HashMap<String, String>,
) -> Command {
    let mut command = Command::new("docker");
    command.arg("compose");

    // Add each compose file with its own -f argument
    for file in compose_files {
        command.arg("-f").arg(file);
    }

    command
        .arg("config")
        .arg("--format")
        .arg("yaml")
        .envs(compose_env_vars);

    command
}
