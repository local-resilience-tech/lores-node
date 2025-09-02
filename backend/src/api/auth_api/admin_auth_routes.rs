use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::config::config_state::LoresNodeConfigState;

use super::{
    admin_user_repo::{AdminUserRepo, GeneratePasswordError},
    auth_backend::{AdminCredentials, AuthError, AuthSession, Credentials},
    UserRef,
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

#[derive(Debug, Clone, Serialize, ToSchema)]
enum AdminLoginError {
    InvalidCredentials,
    NoPasswordSet,
    InternalServerError,
}

#[utoipa::path(
    post, path = "/login",
    request_body(content = AdminCredentials, content_type = "application/json"),
    responses(
        (status = OK, body = UserRef),
        (status = INTERNAL_SERVER_ERROR, body = String),
        (status = UNAUTHORIZED, body = AdminLoginError)
    )
)]
async fn admin_login(
    mut auth_session: AuthSession,
    axum::extract::Json(admin_creds): axum::extract::Json<AdminCredentials>,
) -> impl IntoResponse {
    let creds = Credentials::Admin(admin_creds);

    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(AdminLoginError::InvalidCredentials),
            )
                .into_response();
        }

        Err(e) => {
            eprintln!("Authentication failed: {:?}", e);
            let status = match e {
                axum_login::Error::Backend(AuthError::InvalidCredentials) => (
                    StatusCode::UNAUTHORIZED,
                    Json(AdminLoginError::InvalidCredentials),
                ),
                axum_login::Error::Backend(AuthError::NoPasswordSet) => (
                    StatusCode::UNAUTHORIZED,
                    Json(AdminLoginError::NoPasswordSet),
                ),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(AdminLoginError::InternalServerError),
                ),
            };
            return status.into_response();
        }
    };

    if auth_session.login(&user).await.is_err() {
        eprint!("Failed to log in admin user");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AdminLoginError::InternalServerError),
        )
            .into_response();
    }

    return (StatusCode::OK, Json(UserRef::from_backend_user(&user))).into_response();
}
