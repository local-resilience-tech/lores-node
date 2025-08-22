use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::config::config_state::LoresNodeConfigState;

use super::{
    admin_user_repo::{AdminUserRepo, GeneratePasswordError},
    auth_backend::{AdminCredentials, AuthSession, Credentials},
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(has_admin_password))
        .routes(routes!(generate_admin_password))
        .routes(routes!(admin_login))
}

#[utoipa::path(get, path = "/", responses(
    (status = OK, body = bool),
),)]
async fn has_admin_password(
    Extension(config_state): Extension<LoresNodeConfigState>,
) -> impl IntoResponse {
    let config = config_state.get().await;
    (StatusCode::OK, Json(config.hashed_admin_password.is_some()))
}

#[utoipa::path(post, path = "/", responses(
    (status = CREATED, body = String),
    (status = BAD_REQUEST, body = String),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn generate_admin_password(
    Extension(config_state): Extension<LoresNodeConfigState>,
) -> impl IntoResponse {
    let repo = AdminUserRepo::new(&config_state);

    match repo.generate_and_save_admin_password().await {
        Ok(password) => (StatusCode::CREATED, password).into_response(),
        Err(GeneratePasswordError::PasswordAlreadySet) => {
            eprintln!("Error generating admin password: Password already set");
            (StatusCode::BAD_REQUEST, "Admin password already set").into_response()
        }
        Err(GeneratePasswordError::ServerError) => {
            eprintln!("Error generating admin password: Internal Server Error");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

#[utoipa::path(
    post, path = "/login",
    responses(
        (status = OK, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = String),
        (status = UNAUTHORIZED, body = String)
    ),
    request_body(content = AdminCredentials, content_type = "application/json")
)]
async fn admin_login(
    mut auth_session: AuthSession,
    axum::extract::Json(admin_creds): axum::extract::Json<AdminCredentials>,
) -> impl IntoResponse {
    let creds = Credentials::Admin(admin_creds);

    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err((StatusCode::UNAUTHORIZED, "Invalid credentials").into_response());
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    };

    if auth_session.login(&user).await.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    return Ok((StatusCode::OK, ()).into_response());
}
