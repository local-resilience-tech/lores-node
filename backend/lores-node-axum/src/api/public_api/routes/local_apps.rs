use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    DatabaseState,
    api::helpers::internal_server_error,
    data::entities::LocalAppInstallation,
    local_apps::{find_local_apps, local_app_installations::build_local_app_installations},
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_local_apps))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<LocalAppInstallation>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_local_apps(Extension(db): Extension<DatabaseState>) -> impl IntoResponse {
    let local_apps = match find_local_apps(&db.node_data_pool).await {
        Ok(apps) => apps,
        Err(e) => return internal_server_error(e).into_response(),
    };

    let result = build_local_app_installations(local_apps);

    (StatusCode::OK, Json(result)).into_response()
}
