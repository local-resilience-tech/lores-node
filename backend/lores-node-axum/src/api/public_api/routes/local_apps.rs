use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    DatabaseState,
    api::helpers::internal_server_error,
    data::entities::LocalApp,
    local_apps::find_local_apps,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_local_apps))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<LocalApp>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_local_apps(Extension(db): Extension<DatabaseState>) -> impl IntoResponse {
    match find_local_apps(&db.node_data_pool).await {
        Ok(apps) => (StatusCode::OK, Json(apps)).into_response(),
        Err(e) => internal_server_error(e).into_response(),
    }
}
