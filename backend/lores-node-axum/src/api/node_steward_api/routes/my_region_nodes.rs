use axum::{http::StatusCode, response::IntoResponse, Extension};
use lores_p2panda::operations::LogType;
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    api::{
        auth_api::auth_backend::AuthSession,
        helpers::{bad_request, internal_server_error},
    },
    panda_comms::{
        lores_events::{LoResEventPayload, NodeStatusPostedDataV1, RegionNodeUpdatedDataV1},
        PandaContainer, RegionId,
    },
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(update_this_region_node))
        .routes(routes!(post_region_node_status))
}

#[derive(Deserialize, ToSchema, Debug)]
struct UpdateNodeDetails {
    name: String,
    public_ipv4: Option<String>,
    domain_on_local_network: Option<String>,
    domain_on_internet: Option<String>,
}

fn valid_slug(value: &str) -> bool {
    let slug_regex = regex::Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();
    slug_regex.is_match(value)
}

impl UpdateNodeDetails {
    fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if !valid_slug(&self.name) {
            return Err("Name must be a valid slug".to_string());
        }

        if let Some(public_ipv4) = &self.public_ipv4 {
            if public_ipv4.trim().is_empty() {
                return Err("Public IPv4 cannot be empty".to_string());
            }
        }

        Ok(())
    }
}

#[utoipa::path(
    put,
    path = "/{region_id_string}/my_node",
    params(
        ("region_id_string" = String, Path),
    ),
    responses(
        (status = OK),
        (status = BAD_REQUEST, body = String, description = "Bad Request"),
        (status = INTERNAL_SERVER_ERROR, body = String, description = "Internal Server Error"),
    ),
    request_body(content = UpdateNodeDetails, content_type = "application/json"),
)]
async fn update_this_region_node(
    Extension(panda_container): Extension<PandaContainer>,
    auth_session: AuthSession,
    axum::extract::Path(region_id_string): axum::extract::Path<String>,
    axum::extract::Json(data): axum::extract::Json<UpdateNodeDetails>,
) -> impl IntoResponse {
    println!("update node: {:?}", data);

    // Validate input data
    if let Err(e) = data.validate() {
        return bad_request(e).into_response();
    }

    // Get region_id from path and validate it
    let region_id = match RegionId::from_hex(&region_id_string) {
        Ok(id) => id,
        Err(e) => return bad_request(e).into_response(),
    };

    // Get my node id
    let node_id = match panda_container.get_public_key().await {
        Ok(id) => id,
        Err(e) => return internal_server_error(e).into_response(),
    };

    let event_payload = LoResEventPayload::RegionNodeUpdated(RegionNodeUpdatedDataV1 {
        node_id: node_id.to_hex(),
        region_id: region_id.to_hex(),
        name: Some(data.name.clone()),
        public_ipv4: data.public_ipv4.clone(),
        domain_on_local_network: data.domain_on_local_network.clone(),
        domain_on_internet: data.domain_on_internet.clone(),
    });
    println!("Prepared event payload: {:?}", event_payload);

    // Publish the operation
    let topic_id = PandaContainer::get_region_topic_id(&region_id);
    if let Err(e) = panda_container
        .publish_persisted(topic_id, LogType::Admin, event_payload, auth_session.user)
        .await
    {
        return internal_server_error(e).into_response();
    }

    (StatusCode::OK, ()).into_response()
}

#[derive(Deserialize, ToSchema, Debug)]
struct RegionNodeStatusData {
    pub text: Option<String>,
    pub state: Option<String>,
}

impl RegionNodeStatusData {
    fn validate(&self) -> Result<(), String> {
        if let Some(state) = &self.state {
            let valid_states = ["active", "inactive", "maintenance", "development"];
            if !valid_states.contains(&state.as_str()) {
                return Err(format!(
                    "Invalid state. Valid states are: {}",
                    valid_states.join(", ")
                ));
            }
        }
        Ok(())
    }
}

#[utoipa::path(
    post,
    path = "/{region_id_string}/status",
    params(
        ("region_id_string" = String, Path),
    ),
    responses(
        (status = OK, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = String),
    ),
    request_body(content = RegionNodeStatusData, content_type = "application/json"),
)]
async fn post_region_node_status(
    Extension(panda_container): Extension<PandaContainer>,
    auth_session: AuthSession,
    axum::extract::Path(region_id_string): axum::extract::Path<String>,
    axum::extract::Json(data): axum::extract::Json<RegionNodeStatusData>,
) -> impl IntoResponse {
    println!("post status: {:?}", data);

    // Validate input data
    if let Err(e) = data.validate() {
        return bad_request(e).into_response();
    }

    // Get region_id from path and validate it
    let region_id = match RegionId::from_hex(&region_id_string) {
        Ok(id) => id,
        Err(e) => return bad_request(e).into_response(),
    };

    // Send Operation
    let event_payload = LoResEventPayload::NodeStatusPosted(NodeStatusPostedDataV1 {
        text: data.text.clone(),
        state: data.state.clone(),
    });
    println!("Created event payload: {:?}", event_payload);

    let topic_id = PandaContainer::get_region_topic_id(&region_id);
    if let Err(e) = panda_container
        .publish_persisted(topic_id, LogType::Admin, event_payload, auth_session.user)
        .await
    {
        return internal_server_error(e).into_response();
    }

    (StatusCode::OK, ()).into_response()
}
