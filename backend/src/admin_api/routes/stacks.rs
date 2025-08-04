use axum::{http::StatusCode, response::IntoResponse, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::docker::{docker_stack::docker_stack_ls, DockerStack};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_stacks))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<DockerStack>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_stacks() -> impl IntoResponse {
    let result = docker_stack_ls();

    match result {
        Ok(stacks) => (StatusCode::OK, Json(stacks)).into_response(),
        Err(e) => {
            eprintln!("Error fetching Docker stacks: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(())).into_response();
        }
    }
}
