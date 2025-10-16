use super::utilities::pipe_commands;
use std::{path::PathBuf, process::Command};

pub fn docker_compose_merge_files_no_interpolate(
    compose_files: Vec<PathBuf>,
    output_path: &PathBuf,
) -> Result<(), anyhow::Error> {
    let merged_config = docker_compose_merge_files_no_interpolate_to_string(compose_files)?;

    std::fs::write(output_path, merged_config)
        .map_err(|e| anyhow::anyhow!("Failed to write merged compose file: {}", e))?;

    Ok(())
}

fn docker_compose_merge_files_no_interpolate_to_string(
    compose_files: Vec<PathBuf>,
) -> Result<String, anyhow::Error> {
    if compose_files.is_empty() {
        return Err(anyhow::anyhow!("No compose files provided for merging"));
    }

    let output = pipe_commands(vec![
        docker_compose_merge_files_no_interpolate_command(compose_files),
        make_compose_stack_compatible_command(),
    ])?;

    if !output.status.success() {
        println!(
            "Error: docker compose config failed with status: {}",
            output.status
        );
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        return Err(anyhow::anyhow!(
            "docker compose config failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    println!(
        "Docker compose config successfully executed with status: {}",
        output.status
    );
    let merged_config = String::from_utf8_lossy(&output.stdout).to_string();

    Ok(merged_config)
}

fn docker_compose_merge_files_no_interpolate_command(compose_files: Vec<PathBuf>) -> Command {
    let mut command = Command::new("docker");
    command.arg("compose");

    for file in &compose_files {
        command.arg("-f").arg(file);
    }

    command
        .arg("config")
        .arg("--format")
        .arg("yaml")
        .arg("--no-interpolate");

    command
}

fn make_compose_stack_compatible_command() -> Command {
    let mut command = Command::new("sed");
    command
        .arg("-e")
        .arg("/published:/ s/\"//g")
        .arg("-e")
        .arg("/^name\\:/d");

    command
}
