use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod docker_stack;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DockerStack {
    pub name: String,
    pub services_count: i64,
}
