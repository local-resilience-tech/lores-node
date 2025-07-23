use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{apps::find_installed_apps, projections::entities::App};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(show_all_apps))
        .routes(routes!(register_app))
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<App>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn show_all_apps() -> impl IntoResponse {
    let apps = find_installed_apps();
    (StatusCode::OK, Json(apps)).into_response()
}

#[derive(Deserialize, ToSchema, Debug)]
struct AppIdentifier {
    pub name: String,
}

#[utoipa::path(
    post, path = "/register",
    responses(
        (status = 200, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    ),
    request_body(content = AppIdentifier, content_type = "application/json")
)]
async fn register_app(Json(payload): Json<AppIdentifier>) -> impl IntoResponse {
    println!("Registering app: {:?}", payload);

    (StatusCode::OK, Json(())).into_response()
}
