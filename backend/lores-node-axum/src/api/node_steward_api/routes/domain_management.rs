use std::fmt;

use axum::{Extension, Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use tracing::warn;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    DatabaseState,
    api::{
        auth_api::auth_backend::AuthSession,
        helpers::{bad_request, internal_server_error},
        public_api::{client_events::ClientEvent, realtime::RealtimeState},
    },
    data::{
        entities::{RegisteredDomainProviderDetail, RegisteredDomainProviderRef},
        node_data::registered_domain_providers_repo::{RegisteredDomainProviderRow, RegisteredDomainProvidersRepo},
    },
};

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
        .routes(routes!(register_domain_provider))
        .routes(routes!(get_registered_domain_provider_details))
        .routes(routes!(delete_registered_domain_provider))
        .routes(routes!(update_registered_domain_provider_details))
        .routes(routes!(get_available_domain_providers))
}

#[derive(Deserialize, ToSchema, Debug, Clone)]
pub struct DomainManagementCredentials {
    pub provider: DomainProvider,
    pub username: String,
    pub credential: String,
}

impl RegisteredDomainProviderRow {
    fn from(value: DomainManagementCredentials, registered_by: &str, registration_id: String) -> Self {
        Self {
            registration_id,
            domain_provider: value.provider.to_string(),
            credential: value.credential,
            username: value.username,
            registered_by: registered_by.to_string(),
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
    get, path = "/available_domain_providers",
    responses(
        (status = 200, body = Vec<DomainProvider>),
    )
)]
async fn get_available_domain_providers() -> impl IntoResponse {
    let available_domain_providers = vec![DomainProvider::Serversaurus];

    (StatusCode::OK, Json(available_domain_providers)).into_response()
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
async fn register_domain_provider(
    Extension(db): Extension<DatabaseState>,
    Extension(realtime_state): Extension<RealtimeState>,
    auth_session: AuthSession,
    Json(payload): Json<DomainManagementCredentials>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request(e).into_response();
    }

    let user = match auth_session.user().await {
        Some(user) => user,
        None => return internal_server_error("No auth user found for request").into_response(),
    };

    let row = RegisteredDomainProviderRow::from(payload, &user.id, Uuid::new_v4().to_string());

    let mut tx = match db.node_data_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            warn!("Error starting transaction: {e}");
            return internal_server_error("A database error occured").into_response();
        }
    };

    if let Err(e) = RegisteredDomainProvidersRepo::init().upsert(&mut tx, &row).await {
        warn!("Database error persisting domain provider credentials: {e}");
        return internal_server_error("A database error occured").into_response();
    }

    if let Err(e) = tx.commit().await {
        warn!("Database error committing transaction for domain provider credentials: {e}");
        return internal_server_error("A database error occured").into_response();
    }

    realtime_state
        .broadcast_app_event(ClientEvent::DomainProviderRegistered(RegisteredDomainProviderDetail {
            registration_id: row.registration_id.clone(),
            domain_provider: row.domain_provider,
            username: row.username,
            credential: row.credential,
        }))
        .await;

    (StatusCode::OK, row.registration_id).into_response()
}

#[utoipa::path(
    get, path = "/registered_domains",
    responses(
        (status = 200, body = Vec<RegisteredDomainProviderDetail>),
        (status = INTERNAL_SERVER_ERROR, body = String)
    )
)]
async fn get_registered_domain_provider_details(Extension(db): Extension<DatabaseState>) -> impl IntoResponse {
    let registered_domain_rows = match RegisteredDomainProvidersRepo::init().all(&db.node_data_pool).await {
        Ok(rows) => rows,
        Err(e) => {
            warn!("Database error persisting domain provider credentials: {e}");
            return internal_server_error("A database error occured").into_response();
        }
    };

    let registered_domains: Vec<RegisteredDomainProviderDetail> = registered_domain_rows
        .into_iter()
        .map(|domain| RegisteredDomainProviderDetail {
            registration_id: domain.registration_id,
            domain_provider: domain.domain_provider,
            username: domain.username,
            credential: domain.credential,
        })
        .collect();

    (StatusCode::OK, Json(registered_domains)).into_response()
}

#[utoipa::path(
    delete, path = "/delete",
    request_body(content = RegisteredDomainProviderRef, content_type = "application/json"),
    responses(
        (status = 200, body = ()),
        (status = NOT_FOUND, body = String),
        (status = INTERNAL_SERVER_ERROR, body = String)
    )
)]
async fn delete_registered_domain_provider(
    Extension(db): Extension<DatabaseState>,
    Extension(realtime_state): Extension<RealtimeState>,
    Json(payload): Json<RegisteredDomainProviderRef>,
) -> impl IntoResponse {
    // to do next PR: Call provider API to remove records for node

    match RegisteredDomainProvidersRepo::init()
        .delete(&db.node_data_pool, &payload.registration_id)
        .await
    {
        Ok(false) => (StatusCode::NOT_FOUND, ()).into_response(),
        Err(e) => {
            warn!("Database error persisting domain provider credentials: {e}");
            internal_server_error("A database error occured").into_response()
        }
        Ok(true) => {
            realtime_state
                .broadcast_app_event(ClientEvent::DomainProviderRegistrationDeleted(payload.registration_id))
                .await;

            (StatusCode::OK, ()).into_response()
        }
    }
}

#[utoipa::path(
    put, path = "/{domain_registration_id}/update",
    params(("domain_registration_id" = String, Path)),
    request_body(content = DomainManagementCredentials, content_type = "application/json"),
    responses(
        (status = 200, body = String),
        (status = BAD_REQUEST, body = String),
        (status = NOT_FOUND, body = String),
        (status = INTERNAL_SERVER_ERROR, body = String)
    )
)]
async fn update_registered_domain_provider_details(
    Extension(db): Extension<DatabaseState>,
    Extension(realtime_state): Extension<RealtimeState>,
    auth_session: AuthSession,
    Path(domain_registration_id): Path<String>,
    Json(payload): Json<DomainManagementCredentials>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        return bad_request(e).into_response();
    }

    let user = match auth_session.user().await {
        Some(user) => user,
        None => return internal_server_error("No auth user found for request").into_response(),
    };

    let row = RegisteredDomainProviderRow::from(payload, &user.id, domain_registration_id);

    let mut tx = match db.node_data_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            warn!("Error starting transaction: {e}");
            return internal_server_error("A database error occured").into_response();
        }
    };

    if let Err(e) = RegisteredDomainProvidersRepo::init().upsert(&mut tx, &row).await {
        warn!("Database error persisting domain provider credentials: {e}");
        return internal_server_error("A database error occured").into_response();
    }

    if let Err(e) = tx.commit().await {
        warn!("Database error committing transaction for domain provider credentials: {e}");
        return internal_server_error("A database error occured").into_response();
    }

    realtime_state
        .broadcast_app_event(ClientEvent::DomainProviderRegistrationUpdated(RegisteredDomainProviderDetail {
            registration_id: row.registration_id.clone(),
            domain_provider: row.domain_provider,
            username: row.username,
            credential: row.credential,
        }))
        .await;

    (StatusCode::OK, row.registration_id).into_response()
}
