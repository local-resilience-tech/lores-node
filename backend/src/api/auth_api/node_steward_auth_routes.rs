use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::{
    auth_backend::{AuthError, AuthSession, Credentials, NodeStewardCredentials},
    UserRef,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(node_steward_login))
}

#[derive(Debug, Clone, Serialize, ToSchema)]
enum NodeStewardLoginError {
    InvalidCredentials,
    NoPasswordSet,
    AccountDisabled,
    InternalServerError,
}

#[utoipa::path(
    post, path = "/login",
    request_body(content = NodeStewardCredentials, content_type = "application/json"),
    responses(
        (status = OK, body = UserRef),
        (status = INTERNAL_SERVER_ERROR, body = NodeStewardLoginError),
        (status = UNAUTHORIZED, body = NodeStewardLoginError)
    )
)]
async fn node_steward_login(
    mut auth_session: AuthSession,
    axum::extract::Json(node_steward_creds): axum::extract::Json<NodeStewardCredentials>,
) -> impl IntoResponse {
    let creds = Credentials::NodeSteward(node_steward_creds);

    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(NodeStewardLoginError::InvalidCredentials),
            )
                .into_response());
        }

        Err(e) => {
            eprintln!("Authentication failed: {:?}", e);
            let status = match e {
                axum_login::Error::Backend(AuthError::InvalidCredentials) => (
                    StatusCode::UNAUTHORIZED,
                    Json(NodeStewardLoginError::InvalidCredentials),
                ),
                axum_login::Error::Backend(AuthError::NoPasswordSet) => (
                    StatusCode::UNAUTHORIZED,
                    Json(NodeStewardLoginError::NoPasswordSet),
                ),
                axum_login::Error::Backend(AuthError::AccountDisabled) => (
                    StatusCode::UNAUTHORIZED,
                    Json(NodeStewardLoginError::AccountDisabled),
                ),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(NodeStewardLoginError::InternalServerError),
                ),
            };
            return Err(status.into_response());
        }
    };

    if auth_session.login(&user).await.is_err() {
        eprint!("Failed to log in node steward user");
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(NodeStewardLoginError::InternalServerError),
        )
            .into_response());
    }

    return Ok((StatusCode::OK, Json(UserRef::from_backend_user(&user))).into_response());
}
