use axum::{http::StatusCode, response::IntoResponse, Extension};
use npwg::{generate_password_with_config, PasswordGeneratorConfig};
use password_auth::generate_hash;
use std::collections::HashSet;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::config::LoresNodeConfig;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(generate_admin_password))
}

#[utoipa::path(post, path = "/", responses(
    (status = CREATED, body = String),
    (status = BAD_REQUEST, body = String),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn generate_admin_password(
    Extension(config): Extension<LoresNodeConfig>,
) -> impl IntoResponse {
    if config.hashed_admin_password.is_some() {
        return (StatusCode::BAD_REQUEST, "Admin password already set").into_response();
    }

    // Generate a new admin password
    let mut pw_config = PasswordGeneratorConfig::new();
    pw_config.length = 20;
    pw_config.excluded_chars = HashSet::from([':', ';', '"']);
    let pw_result = generate_password_with_config(&pw_config).await;

    let password: String = match pw_result {
        Err(err) => {
            eprintln!("Error generating password: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response();
        }
        Ok(pw) => pw,
    };

    // Hash the password and store in config
    // let hashed_password = generate_hash(&password);
    // config.hashed_admin_password = Some(hashed_password);
    // match config.try_save() {
    //     Ok(_) => (StatusCode::CREATED, password).into_response(),
    //     Err(err) => {
    //         eprintln!("Failed to save config: {}", err);
    //         (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
    //     }
    // }

    (StatusCode::CREATED, password).into_response()
}
