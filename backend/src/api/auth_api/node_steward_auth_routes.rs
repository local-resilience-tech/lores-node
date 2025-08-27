use axum::{http::StatusCode, response::IntoResponse, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use super::{
    auth_backend::{AuthError, AuthSession, Credentials, NodeStewardCredentials},
    UserRef,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(node_steward_login))
}

#[utoipa::path(
    post, path = "/login",
    request_body(content = NodeStewardCredentials, content_type = "application/json"),
    responses(
        (status = OK, body = UserRef),
        (status = INTERNAL_SERVER_ERROR, body = String),
        (status = UNAUTHORIZED, body = String)
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
                "Invalid credentials for node steward",
            )
                .into_response());
        }

        Err(e) => {
            eprintln!("Authentication failed: {:?}", e);
            let status = match e {
                axum_login::Error::Backend(AuthError::InvalidCredentials) => (
                    StatusCode::UNAUTHORIZED,
                    "Invalid credentials for node steward",
                ),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            };
            return Err(status.into_response());
        }
    };

    if auth_session.login(&user).await.is_err() {
        eprint!("Failed to log in node steward user");
        return Err(StatusCode::INTERNAL_SERVER_ERROR.into_response());
    }

    return Ok((StatusCode::OK, Json(UserRef::from_backend_user(&user))).into_response());
}
