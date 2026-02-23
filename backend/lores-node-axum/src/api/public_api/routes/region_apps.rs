use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    data::{entities::RegionAppWithInstallations, projections_read::apps::AppsReadRepo},
    DatabaseState,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_region_apps))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<RegionAppWithInstallations>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_region_apps(Extension(db): Extension<DatabaseState>) -> impl IntoResponse {
    let repo = AppsReadRepo::init();

    repo.all_with_installations(&db.projections_pool)
        .await
        .map(|nodes| (StatusCode::OK, Json(nodes)))
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(())))
        .into_response()
}
