use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use password_auth::generate_hash;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    data::node_data::node_stewards::{NodeStewardIdentifier, NodeStewardsRepo},
    DatabaseState,
};

use super::{
    auth_backend::{AuthError, AuthSession, Credentials, NodeStewardCredentials},
    UserRef,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_current_user))
        .routes(routes!(node_steward_login))
        .routes(routes!(node_steward_set_password))
}

#[derive(Debug, Clone, Serialize, ToSchema)]
struct NodeStewardUser {
    pub id: String,
    pub name: String,
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = OK, body = Option<NodeStewardUser>),
        (status = INTERNAL_SERVER_ERROR, body = ()),
    )
)]
async fn get_current_user(
    Extension(db): Extension<DatabaseState>,
    auth_session: AuthSession,
) -> impl IntoResponse {
    let auth_user = match auth_session.user {
        Some(user) => user,
        None => {
            eprintln!("Failed to get current user");
            return (StatusCode::OK, Json(Option::<NodeStewardUser>::None)).into_response();
        }
    };

    let repo = NodeStewardsRepo::init();

    // Fetch node steward by ID
    let id = NodeStewardIdentifier {
        id: auth_user.id.clone(),
    };
    let steward = match repo.find(&db.node_data_pool, &id).await {
        Ok(Some(steward)) => steward,
        Ok(None) => {
            return (StatusCode::OK, Json(Option::<NodeStewardUser>::None)).into_response();
        }
        Err(e) => {
            eprintln!("Error finding node steward: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(NodeStewardSetPasswordError::InternalServerError),
            )
                .into_response();
        }
    };

    let node_steward_user = NodeStewardUser {
        id: steward.id.clone(),
        name: steward.name.clone(),
    };

    (StatusCode::OK, Json(Some(node_steward_user))).into_response()
}

#[derive(Debug, Clone, Serialize, ToSchema)]
enum NodeStewardLoginError {
    InvalidCredentials,
    NoPasswordSet,
    // AccountDisabled,
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
            return (
                StatusCode::UNAUTHORIZED,
                Json(NodeStewardLoginError::InvalidCredentials),
            )
                .into_response();
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
                // axum_login::Error::Backend(AuthError::AccountDisabled) => (
                //     StatusCode::UNAUTHORIZED,
                //     Json(NodeStewardLoginError::AccountDisabled),
                // ),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(NodeStewardLoginError::InternalServerError),
                ),
            };
            return status.into_response();
        }
    };

    if auth_session.login(&user).await.is_err() {
        eprint!("Failed to log in node steward user");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(NodeStewardLoginError::InternalServerError),
        )
            .into_response();
    }

    return (StatusCode::OK, Json(UserRef::from_backend_user(&user))).into_response();
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
pub struct NodeStewardSetPasswordRequest {
    pub id: String,
    pub token: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum NodeStewardSetPasswordError {
    InvalidId,
    InvalidToken,
    TokenExpired,
    InvalidNewPassword,
    InternalServerError,
}

#[utoipa::path(
    post, path = "/set_password",
    request_body(content = NodeStewardSetPasswordRequest, content_type = "application/json"),
    responses(
        (status = OK, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = NodeStewardSetPasswordError),
        (status = UNAUTHORIZED, body = NodeStewardSetPasswordError)
    )
)]
async fn node_steward_set_password(
    Extension(db): Extension<DatabaseState>,
    axum::extract::Json(input): axum::extract::Json<NodeStewardSetPasswordRequest>,
) -> impl IntoResponse {
    let repo = NodeStewardsRepo::init();

    // Fetch node steward by ID
    let id = NodeStewardIdentifier { id: input.id };
    let steward = match repo.find(&db.node_data_pool, &id).await {
        Ok(Some(steward)) => steward,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(NodeStewardSetPasswordError::InvalidId),
            )
                .into_response();
        }
        Err(e) => {
            eprintln!("Failed to find node steward: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(NodeStewardSetPasswordError::InternalServerError),
            )
                .into_response();
        }
    };

    // Check that the token is valid and not empty or missing
    if !steward.token_equals(&input.token) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(NodeStewardSetPasswordError::InvalidToken),
        )
            .into_response();
    }

    // Check that the token hasn't expired
    if steward.token_expired() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(NodeStewardSetPasswordError::TokenExpired),
        )
            .into_response();
    }

    // Check that the new password is valid
    if !password_is_valid(&input.new_password) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(NodeStewardSetPasswordError::InvalidNewPassword),
        )
            .into_response();
    }

    // Hash the password and save it
    let hashed_password = generate_hash(&input.new_password);
    let result = repo
        .update_password_and_clear_token(&db.node_data_pool, &id, &hashed_password)
        .await;

    match result {
        Ok(_) => (StatusCode::OK, ()).into_response(),
        Err(e) => {
            eprintln!("Failed to update node steward password: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(NodeStewardSetPasswordError::InternalServerError),
            )
                .into_response()
        }
    }
}

fn password_is_valid(password: &str) -> bool {
    password.len() >= 8
}
