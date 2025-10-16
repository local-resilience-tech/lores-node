use std::process::{Command, Stdio};

/// Connects multiple commands via pipes, executing them in sequence and piping stdout to the next command's stdin.
/// Returns the output of the last command in the chain.
pub fn pipe_commands(mut commands: Vec<Command>) -> Result<std::process::Output, anyhow::Error> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_commands_concat_three_words() {
        // First command: echo "one"
        let mut echo_cmd = Command::new("echo");
        echo_cmd.arg("one");

        // Second command: sed 's/$/two/'
        let mut sed_cmd = Command::new("sed");
        sed_cmd.arg("s/$/two/");

        // Third command: sed 's/$/three/'
        let mut sed_cmd2 = Command::new("sed");
        sed_cmd2.arg("s/$/three/");

        let commands = vec![echo_cmd, sed_cmd, sed_cmd2];

        let result = pipe_commands(commands).expect("Failed to execute piped commands");

        // Should return "onetwothree" (plus a newline)
        let output = String::from_utf8_lossy(&result.stdout).trim().to_string();
        assert_eq!(output, "onetwothree");
    }
}
