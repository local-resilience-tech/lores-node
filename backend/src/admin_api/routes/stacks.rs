use axum::{http::StatusCode, response::IntoResponse, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::projections::entities::Stack;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_stacks))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<Stack>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_stacks() -> impl IntoResponse {
    // This is a placeholder implementation. Replace with actual logic to fetch stacks.
    let stacks = vec![
        Stack {
            name: "Stack1".to_string(),
        },
        Stack {
            name: "Stack2".to_string(),
        },
    ];

    (StatusCode::OK, Json(stacks))
}
