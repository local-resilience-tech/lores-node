use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use sqlx::SqlitePool;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::projections::{entities::RegionApp, projections_read::apps::AppsReadRepo};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(show_all_region_apps))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<RegionApp>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn show_all_region_apps(Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
    let repo = AppsReadRepo::init();

    repo.all(&pool)
        .await
        .map(|nodes| (StatusCode::OK, Json(nodes)))
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(())))
        .into_response()
}
