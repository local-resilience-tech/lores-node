use axum::{Json, http::StatusCode, response::IntoResponse};
use coop_cloud_docker_apps as cca;
use serde::Serialize;
use tracing::warn;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(list_stacks))
}

#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct DockerService {
    pub id: String,
    pub name: String,
    pub image: String,
    pub node_name: String,
    pub current_state: String,
    pub current_state_duration: String,
}

impl From<cca::DockerService> for DockerService {
    fn from(s: cca::DockerService) -> Self {
        Self {
            id: s.id,
            name: s.name,
            image: s.image,
            node_name: s.node_name,
            current_state: s.current_state,
            current_state_duration: s.current_state_duration,
        }
    }
}

#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct DockerStackWithServices {
    pub name: String,
    pub services: Vec<DockerService>,
}

impl From<cca::DockerStackWithServices> for DockerStackWithServices {
    fn from(s: cca::DockerStackWithServices) -> Self {
        Self {
            name: s.name,
            services: s.services.into_iter().map(DockerService::from).collect(),
        }
    }
}

#[utoipa::path(get, path = "/", responses(
    (status = 200, body = Vec<DockerStackWithServices>),
    (status = INTERNAL_SERVER_ERROR, body = ()),
),)]
async fn list_stacks() -> impl IntoResponse {
    let result = cca::docker_stacks_with_services();

    match result {
        Ok(stacks) => {
            let stacks: Vec<DockerStackWithServices> = stacks.into_iter().map(DockerStackWithServices::from).collect();
            (StatusCode::OK, Json(stacks)).into_response()
        }
        Err(e) => {
            warn!("Error fetching Docker stacks: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(())).into_response();
        }
    }
}
