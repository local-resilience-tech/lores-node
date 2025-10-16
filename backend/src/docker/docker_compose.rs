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
    let config_command = docker_compose_output_config_command(compose_files, compose_env_vars);

    println!(
        "Running compose config command: {:?} {:?}",
        config_command.get_program(),
        config_command.get_args()
    );

    // Create a sed command that reads from config output
    let mut sed_command = Command::new("sed");
    sed_command
        .arg("-e")
        .arg("/published:/ s/\"//g")
        .arg("-e")
        .arg("/^name\\:/d");

    // Pipe the commands together
    let sed_output = pipe_commands(vec![config_command, sed_command])?;

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

/// Connects multiple commands via pipes, executing them in sequence and piping stdout to the next command's stdin.
/// Returns the output of the last command in the chain.
fn pipe_commands(mut commands: Vec<Command>) -> Result<std::process::Output, anyhow::Error> {
    if commands.is_empty() {
        return Err(anyhow::anyhow!("No commands provided to pipe"));
    }

    if commands.len() == 1 {
        return commands[0]
            .output()
            .map_err(|e| anyhow::anyhow!("Failed to run single command: {}", e));
    }

    // Set up all intermediate commands to pipe
    let mut child_processes = Vec::with_capacity(commands.len() - 1);

    for i in 0..commands.len() - 1 {
        commands[i].stdout(Stdio::piped());

        let mut child = commands[i]
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to start command {}: {}", i, e))?;

        let stdout = child
            .stdout
            .take()
            .expect(&format!("Failed to open command {} stdout", i));

        // Next command takes this command's stdout as its stdin
        commands[i + 1].stdin(Stdio::from(stdout));

        child_processes.push(child);
    }

    // Run the final command and get its output
    commands
        .last_mut()
        .unwrap()
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run final command: {}", e))
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
