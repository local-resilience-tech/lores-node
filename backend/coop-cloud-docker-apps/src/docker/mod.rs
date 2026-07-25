pub mod docker_service;
pub mod docker_stack;
mod helpers;

#[derive(Debug, Clone)]
pub struct DockerStack {
    pub name: String,
    pub services_count: i64,
}
