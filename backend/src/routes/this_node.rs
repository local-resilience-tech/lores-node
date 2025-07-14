use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::SqlitePool;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::repos::{entities::Node, this_node::ThisNodeRepo};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(show_this_node))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Option<Node>),
    (status = INTERNAL_SERVER_ERROR, body = String, description = "Internal Server Error"),
))]
async fn show_this_node(Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
    let repo = ThisNodeRepo::init();

    let node = repo.find(&pool).await;

    match node {
        Ok(node) => (StatusCode::OK, Json(node)).into_response(),
        Err(err) => {
            eprintln!("Error fetching node: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}
