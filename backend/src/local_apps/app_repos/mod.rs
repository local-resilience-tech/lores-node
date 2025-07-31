use super::app_definitions::AppDefinition;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod git;
pub mod installed;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AppRepoSource {
    pub name: String,
    pub git_url: String,
}

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct AppRepo {
    pub name: String,
    pub apps: Vec<AppDefinition>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct AppRepoReference {
    pub name: String,
}
