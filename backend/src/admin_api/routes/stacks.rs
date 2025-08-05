use axum::{http::StatusCode, response::IntoResponse, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::docker::{docker_stacks_with_services, DockerStackWithServices};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_stacks))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<DockerStackWithServices>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_stacks() -> impl IntoResponse {
    let result = docker_stacks_with_services();

    match result {
        Ok(stacks) => (StatusCode::OK, Json(stacks)).into_response(),
        Err(e) => {
            eprintln!("Error fetching Docker stacks: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(())).into_response();
        }
    }
}
