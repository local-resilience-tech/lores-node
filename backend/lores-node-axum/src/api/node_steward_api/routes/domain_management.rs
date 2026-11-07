use std::fmt;

use axum::{Extension, Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::warn;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    DatabaseState,
    api::helpers::{bad_request, internal_server_error},
    data::{
        entities::RegisteredDomainData,
        node_data::registered_domain_providers_repo::{RegisteredDomainProvidersRepo, RegisteredDomainProvidersRow},
    },
};

// to do - move this
#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub enum DomainProvider {
    Serversaurus,
}

impl fmt::Display for DomainProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(register_provider))
        .routes(routes!(get_registered_domain))
}

#[derive(Deserialize, ToSchema, Debug, Clone)]
pub struct DomainManagementCredentials {
    pub provider: DomainProvider,
    pub username: String,
    pub credential: String,
}

impl From<DomainManagementCredentials> for RegisteredDomainProvidersRow {
    fn from(value: DomainManagementCredentials) -> Self {
        Self {
            registration_id: Uuid::new_v4().to_string(),
            domain_provider: value.provider.to_string(),
            credential: value.credential, // to encrypted?
            username: value.username,
        }
    }
}

impl DomainManagementCredentials {
    pub fn validate(&self) -> Result<(), String> {
        if self.username.trim().is_empty() {
            return Err("username must not be empty".to_string());
        }
        if self.credential.trim().is_empty() {
            return Err("credential must not be empty".to_string());
        }

        Ok(())
    }
}

#[utoipa::path(
    post, path = "/register",
    request_body(content = DomainManagementCredentials, content_type = "application/json"),
    responses(
        (status = 200, body = String),
        (status = BAD_REQUEST, body = String),
        (status = INTERNAL_SERVER_ERROR, body = String)
    )
)]
async fn register_provider(Extension(db): Extension<DatabaseState>, Json(payload): Json<DomainManagementCredentials>) -> impl IntoResponse {
    if let Err(error) = payload.validate() {
        return bad_request(error).into_response();
    }

    let repo = RegisteredDomainProvidersRepo::init();
    let row = RegisteredDomainProvidersRow::from(payload);

    if let Err(e) = repo.upsert(&db.node_data_pool, &row).await {
        warn!("Database error persisting domain provider credentials: {e}");
        return internal_server_error("A database error occured").into_response();
    }

    (StatusCode::OK, row.registration_id).into_response()
}

#[utoipa::path(
    get, path = "/{domain_registration_id}",
    params(("domain_registration_id", description = "id for the domain registration to query")),
    responses(
        (status = 200, body = RegisteredDomainData),
        (status = NOT_FOUND, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = String)
    )
)]
async fn get_registered_domain(Path(domain_registration_id): Path<String>, Extension(db): Extension<DatabaseState>) -> impl IntoResponse {
    let repo = RegisteredDomainProvidersRepo::init();

    let registered_domain = match repo.find(&db.node_data_pool, &domain_registration_id).await {
        Ok(Some(domain_row)) => domain_row,
        Ok(None) => return (StatusCode::NOT_FOUND, ()).into_response(),
        Err(e) => {
            warn!("Database error persisting domain provider credentials: {e}");
            return internal_server_error("A database error occured").into_response();
        }
    };

    // Next PR
    // call the credential provider API, add the data to the below

    let domain_data = RegisteredDomainData {
        domain_provider: registered_domain.domain_provider,
        credential: registered_domain.credential,
        username: registered_domain.username,
    };

    (StatusCode::OK, Json(domain_data)).into_response()
}

// to do - we'll need an endpoint for querying all
// the domain registrations (need to wait to hear if all users can view all registrations?)
