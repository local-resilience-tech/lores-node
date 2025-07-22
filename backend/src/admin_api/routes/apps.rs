use axum::{http::StatusCode, response::IntoResponse, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{apps::find_installed_apps, projections::entities::App};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(show_all_apps))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<App>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn show_all_apps() -> impl IntoResponse {
    let apps = find_installed_apps();
    (StatusCode::OK, Json(apps)).into_response()
}
