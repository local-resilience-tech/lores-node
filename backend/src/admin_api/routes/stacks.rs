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
    let stacks = vec![
        DockerStack {
            name: "Stack1".to_string(),
        },
        DockerStack {
            name: "Stack2".to_string(),
        },
    ];

    (StatusCode::OK, Json(stacks))
}
