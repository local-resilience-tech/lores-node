use axum::{http::StatusCode, response::IntoResponse, Extension};
use npwg::{generate_password_with_config, PasswordGeneratorConfig};
use password_auth::generate_hash;
use std::collections::HashSet;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::config::config_state::LoresNodeConfigState;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(generate_admin_password))
}

#[utoipa::path(post, path = "/", responses(
    (status = CREATED, body = String),
    (status = BAD_REQUEST, body = String),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn generate_admin_password(
    Extension(config_state): Extension<LoresNodeConfigState>,
) -> impl IntoResponse {
    let config = config_state.get().await;
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
    let hashed_password = generate_hash(&password);
    let update_result = config_state
        .update(|config| {
            let mut result = config.clone();
            result.hashed_admin_password = Some(hashed_password);
            result
        })
        .await;

    match update_result {
        Ok(_) => (StatusCode::CREATED, password).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response(),
    }
}
