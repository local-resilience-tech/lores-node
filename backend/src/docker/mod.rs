use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod docker_compose;
pub mod docker_stack;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct DockerStack {
    pub name: String,
    pub services_count: i64,
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

#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct DockerStackWithServices {
    pub name: String,
    pub services: Vec<DockerService>,
}

pub fn docker_stacks_with_services() -> Result<Vec<DockerStackWithServices>, anyhow::Error> {
    let stacks = docker_stack::docker_stack_ls()?;
    let mut stacks_with_services = Vec::new();

    for stack in stacks {
        let services = docker_stack::docker_stack_ps(&stack.name)?;
        stacks_with_services.push(DockerStackWithServices {
            name: stack.name,
            services,
        });
    }

    Ok(stacks_with_services)
}
