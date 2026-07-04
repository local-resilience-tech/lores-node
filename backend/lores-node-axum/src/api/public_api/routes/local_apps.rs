use axum::{Json, http::StatusCode, response::IntoResponse};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    data::entities::LocalAppInstallation, local_apps::stack_apps::find_deployed_local_apps,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_local_apps))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<LocalAppInstallation>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_local_apps() -> impl IntoResponse {
    let apps = find_deployed_local_apps()
        .into_iter()
        .map(|app| LocalAppInstallation {
            app,
            region_id: None,
        })
        .collect::<Vec<_>>();
    (StatusCode::OK, Json(apps)).into_response()
}
