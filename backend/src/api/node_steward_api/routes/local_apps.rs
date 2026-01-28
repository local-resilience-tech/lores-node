use axum::{http::StatusCode, response::IntoResponse, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::local_apps::installed_apps::AppReference;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(register_app))
}

#[utoipa::path(
    post, path = "/register",
    request_body(content = AppReference, content_type = "application/json"),
    responses(
        (status = 200, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    )
)]
async fn register_app(
    // Extension(panda_container): Extension<P2PandaContainer>,
    // auth_session: AuthSession,
    Json(payload): Json<AppReference>,
) -> impl IntoResponse {
    // match load_local_app_details(&payload) {
    //     Some(app) => {
    //         let event_payload = LoResEventPayload::AppRegistered(AppRegisteredDataV1 {
    //             name: app.name.clone(),
    //             version: app.version.clone(),
    //         });
    //         let publish_result = panda_container
    //             .publish_persisted(event_payload, auth_session.user)
    //             .await;
    //         match publish_result {
    //             Ok(_) => {
    //                 event!(tracing::Level::INFO, "App registered: {}", app.name);
    //                 (StatusCode::OK, ())
    //             }
    //             Err(e) => {
    //                 eprintln!("Failed to publish app registration event: {}", e);
    //                 (StatusCode::INTERNAL_SERVER_ERROR, ())
    //             }
    //         }
    //     }
    //     None => {
    //         eprintln!("Failed to load app configuration for: {}", payload.app_name);
    //         (StatusCode::INTERNAL_SERVER_ERROR, ())
    //     }
    // }
    // .into_response()

    println!("TODO: register_app called for app: {}", payload.app_name);
    (StatusCode::OK, ()).into_response()
}
