use axum::{http::StatusCode, response::IntoResponse, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{data::entities::LocalApp, local_apps::stack_apps::find_deployed_local_apps};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_local_apps))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<LocalApp>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_local_apps() -> impl IntoResponse {
    let apps = find_deployed_local_apps();
    (StatusCode::OK, Json(apps)).into_response()
}
