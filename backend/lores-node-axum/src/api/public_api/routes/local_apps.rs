use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    DatabaseState,
    api::helpers::internal_server_error,
    data::entities::LocalAppInstallation,
    local_apps::{
        local_app_installations::build_local_app_installations,
        stack_apps::find_deployed_local_apps,
    },
    panda_comms::PandaContainer,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_local_apps))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<LocalAppInstallation>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_local_apps(
    Extension(panda_container): Extension<PandaContainer>,
    Extension(db): Extension<DatabaseState>,
) -> impl IntoResponse {
    let node_id = match panda_container.get_public_key().await {
        Ok(id) => id,
        Err(e) => return internal_server_error(e).into_response(),
    };
    let node_id_hex = node_id.to_hex();

    let local_apps = find_deployed_local_apps();

    let result =
        match build_local_app_installations(&db.projections_pool, &node_id_hex, local_apps).await {
            Ok(installations) => installations,
            Err(e) => return internal_server_error(e).into_response(),
        };

    (StatusCode::OK, Json(result)).into_response()
}
