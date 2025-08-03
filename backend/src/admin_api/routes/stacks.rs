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

fn docker_stack_ls() -> Result<Vec<DockerStack>, anyhow::Error> {
    // use rust to run the command `docker stack ls`
    // and parse the output into a Vec<DockerStack>

    let output = Command::new("docker")
        .arg("stack")
        .arg("ls")
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to execute command: {}", e))?;

    let stdout_string = String::from_utf8(output.stdout)
        .map_err(|e| anyhow::anyhow!("Failed to convert output to string: {}", e))?;

    let stacks: Vec<DockerStack> = stdout_string
        .lines()
        .map(|line| DockerStack {
            name: line.to_string(),
        })
        .collect();

    Ok(stacks)
}
