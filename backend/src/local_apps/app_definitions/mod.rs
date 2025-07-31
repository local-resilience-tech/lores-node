use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod config;
pub mod fs;

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct AppDefinitionReference {
    pub repo_name: String,
    pub app_name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AppDefinition {
    pub name: String,
    pub version: String,
}

// pub struct RepoAppReference {
//     pub repo_name: String,
//     pub app_name: String,
// }
