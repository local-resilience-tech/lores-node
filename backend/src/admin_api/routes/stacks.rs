use std::process::Command;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_stacks))
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DockerStack {
    pub name: String,
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<DockerStack>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_stacks() -> impl IntoResponse {
    // This is a placeholder implementation. Replace with actual logic to fetch stacks.
    let result = docker_stack_ls();

    match result {
        Ok(stacks) => (StatusCode::OK, Json(stacks)).into_response(),
        Err(e) => {
            eprintln!("Error fetching Docker stacks: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(())).into_response();
        }
    }
}

fn parse_table(output: &str) -> Vec<Vec<String>> {
    let mut lines = output.lines();
    let header = match lines.next() {
        Some(h) => h,
        None => return vec![],
    };

    let col_starts: Vec<usize> = header
        .char_indices()
        .filter_map(|(i, c)| {
            if i == 0 || (c != ' ' && header.chars().nth(i - 1) == Some(' ')) {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            col_starts
                .iter()
                .enumerate()
                .map(|(i, &start)| {
                    let end = if i + 1 < col_starts.len() {
                        col_starts[i + 1]
                    } else {
                        line.len()
                    };
                    line.get(start..end).unwrap_or("").trim().to_string()
                })
                .collect()
        })
        .collect()
}

fn docker_stack_ls() -> Result<Vec<DockerStack>, anyhow::Error> {
    let output = Command::new("docker")
        .arg("stack")
        .arg("ls")
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute command: {}", e))?;

    let stdout_string = String::from_utf8(output.stdout)
        .map_err(|e| anyhow::anyhow!("Failed to convert output to string: {}", e))?;

    let table = parse_table(&stdout_string);

    let stacks: Vec<DockerStack> = table
        .into_iter()
        .filter_map(|row| row.get(0).cloned())
        .map(|name| DockerStack { name })
        .collect();

    Ok(stacks)
}
