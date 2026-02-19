use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    api::auth_api::auth_backend::AuthSession,
    data::entities::RegionNode,
    panda_comms::{
        lores_events::{
            LoResEventPayload, NodeAnnouncedDataV1, NodeStatusPostedDataV1, NodeUpdatedDataV1,
        },
        panda_node_container::PandaNodeContainer,
    },
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(create_this_node))
        .routes(routes!(update_this_node))
        .routes(routes!(post_node_status))
}

#[derive(Deserialize, ToSchema, Debug)]
struct CreateNodeDetails {
    name: String,
}

#[utoipa::path(
    post,
    path = "/",
    responses(
        (status = CREATED, body = RegionNode),
        (status = INTERNAL_SERVER_ERROR, body = String, description = "Internal Server Error"),
    ),
    request_body(content = CreateNodeDetails, content_type = "application/json"),
)]
async fn create_this_node(
    Extension(panda_container): Extension<PandaNodeContainer>,
    auth_session: AuthSession,
    axum::extract::Json(data): axum::extract::Json<CreateNodeDetails>,
) -> impl IntoResponse {
    let event_payload = LoResEventPayload::NodeAnnounced(NodeAnnouncedDataV1 {
        name: data.name.clone(),
    });
    println!("Created event payload: {:?}", event_payload);

    let result = panda_container
        .publish_persisted(event_payload, auth_session.user)
        .await;

    match result {
        Ok(_) => {
            let node = RegionNode {
                id: "1".to_string(),
                name: data.name.clone(),
                public_ipv4: None,
                domain_on_local_network: None,
                domain_on_internet: None,
            };
            (StatusCode::CREATED, Json(node)).into_response()
        }
        Err(e) => {
            eprintln!("Error publishing event: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response()
        }
    }
}

#[derive(Deserialize, ToSchema, Debug)]
struct UpdateNodeDetails {
    name: String,
    public_ipv4: String,
    domain_on_local_network: Option<String>,
    domain_on_internet: Option<String>,
}

#[utoipa::path(
    put,
    path = "/",
    responses(
        (status = OK, body = RegionNode),
        (status = INTERNAL_SERVER_ERROR, body = String, description = "Internal Server Error"),
    ),
    request_body(content = UpdateNodeDetails, content_type = "application/json"),
)]
async fn update_this_node(
    Extension(panda_container): Extension<PandaNodeContainer>,
    auth_session: AuthSession,
    axum::extract::Json(data): axum::extract::Json<UpdateNodeDetails>,
) -> impl IntoResponse {
    println!("update node: {:?}", data);

    let event_payload = LoResEventPayload::NodeUpdated(NodeUpdatedDataV1 {
        name: data.name.clone(),
        public_ipv4: data.public_ipv4.clone(),
        domain_on_local_network: data.domain_on_local_network.clone(),
        domain_on_internet: data.domain_on_internet.clone(),
    });
    println!("Prepared event payload: {:?}", event_payload);

    let result = panda_container
        .publish_persisted(event_payload, auth_session.user)
        .await;

    match result {
        Ok(_) => {
            let node = RegionNode {
                id: "1".to_string(),
                name: data.name.clone(),
                public_ipv4: Some(data.public_ipv4.clone()),
                domain_on_local_network: data.domain_on_local_network.clone(),
                domain_on_internet: data.domain_on_internet.clone(),
            };
            (StatusCode::OK, Json(node)).into_response()
        }
        Err(e) => {
            eprintln!("Error publishing event: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response()
        }
    }
}

#[derive(Deserialize, ToSchema, Debug)]
struct NodeStatusData {
    pub text: Option<String>,
    pub state: Option<String>,
}

#[utoipa::path(
    post,
    path = "/status",
    responses(
        (status = OK, body = ()),
        (status = INTERNAL_SERVER_ERROR, body = String),
    ),
    request_body(content = NodeStatusData, content_type = "application/json"),
)]
async fn post_node_status(
    Extension(panda_container): Extension<PandaNodeContainer>,
    auth_session: AuthSession,
    axum::extract::Json(data): axum::extract::Json<NodeStatusData>,
) -> impl IntoResponse {
    println!("post status: {:?}", data);

    let event_payload = LoResEventPayload::NodeStatusPosted(NodeStatusPostedDataV1 {
        text: data.text.clone(),
        state: data.state.clone(),
    });
    println!("Created event payload: {:?}", event_payload);

    let result = panda_container
        .publish_persisted(event_payload, auth_session.user)
        .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(())).into_response(),
        Err(e) => {
            eprintln!("Error publishing event: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response()
        }
    }
}
