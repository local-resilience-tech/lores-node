use axum::{http::StatusCode, response::IntoResponse, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::local_apps::app_repos::{self, AppRepo};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_app_repos))
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, body = [AppRepo]),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    ),
)]
async fn list_app_repos() -> impl IntoResponse {
    let repos = app_repos::fs::list_installed_app_repos();

    (StatusCode::OK, Json(repos))
}
