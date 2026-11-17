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
        entities::{RegisteredDomainDetail, RegisteredDomainRef},
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
        .routes(routes!(get_registered_domain_details))
        .routes(routes!(delete_domain_registration))
        .routes(routes!(update_domain_registration))
}

#[derive(Deserialize, ToSchema, Debug, Clone)]
pub struct DomainManagementCredentials {
    pub provider: DomainProvider,
    pub username: String,
    pub credential: String,
}

impl RegisteredDomainProvidersRow {
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
    post, path = "/register",
    request_body(content = DomainManagementCredentials, content_type = "application/json"),
    responses(
        (status = 200, body = String),
        (status = BAD_REQUEST, body = String),
        (status = INTERNAL_SERVER_ERROR, body = String)
    )
)]
async fn register_provider(
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

    let row = RegisteredDomainProvidersRow::from(payload, &user.id, Uuid::new_v4().to_string());

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
        .broadcast_app_event(ClientEvent::DomainRegistered(RegisteredDomainDetail {
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
        (status = 200, body = Vec<RegisteredDomainDetail>),
        (status = INTERNAL_SERVER_ERROR, body = String)
    )
)]
async fn get_registered_domain_details(Extension(db): Extension<DatabaseState>) -> impl IntoResponse {
    let registered_domain_rows = match RegisteredDomainProvidersRepo::init().all(&db.node_data_pool).await {
        Ok(rows) => rows,
        Err(e) => {
            // to do - chuck this into a function or something?
            warn!("Database error persisting domain provider credentials: {e}");
            return internal_server_error("A database error occured").into_response();
        }
    };

    let registered_domains: Vec<RegisteredDomainDetail> = registered_domain_rows
        .into_iter()
        .map(|row| RegisteredDomainDetail {
            registration_id: row.registration_id,
            domain_provider: row.domain_provider,
            username: row.username,
            credential: row.credential,
        })
        .collect();

    (StatusCode::OK, Json(registered_domains)).into_response()
}

#[utoipa::path(
    delete, path = "/delete",
    request_body(content = RegisteredDomainRef, content_type = "application/json"),
    responses(
        (status = 200, body = ()),
        (status = NOT_FOUND, body = String),
        (status = INTERNAL_SERVER_ERROR, body = String)
    )
)]
async fn delete_domain_registration(
    Extension(db): Extension<DatabaseState>,
    Extension(realtime_state): Extension<RealtimeState>,
    Json(payload): Json<RegisteredDomainRef>,
) -> impl IntoResponse {
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
                .broadcast_app_event(ClientEvent::DomainRegistrationDeleted(payload.registration_id))
                .await;

            (StatusCode::OK, ()).into_response()
        }
    }

    // Should we be removing domains with the provider here
    // or just the registration at the node level?
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
async fn update_domain_registration(
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

    let row = RegisteredDomainProvidersRow::from(payload, &user.id, domain_registration_id);

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
        .broadcast_app_event(ClientEvent::DomainRegistrationUpdated(RegisteredDomainDetail {
            registration_id: row.registration_id.clone(),
            domain_provider: row.domain_provider,
            username: row.username,
            credential: row.credential,
        }))
        .await;

    (StatusCode::OK, row.registration_id).into_response()
}
