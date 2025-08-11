use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod fs;
pub mod git;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AppRepoSource {
    pub name: String,
    pub git_url: String,
}

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct AppDefinition {
    pub name: String,
    pub latest_version: Option<String>,
    pub versions: Vec<String>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct AppRepo {
    pub name: String,
    pub git_url: String,
    pub apps: Vec<AppDefinition>,
}

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct AppRepoReference {
    pub repo_name: String,
}

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct AppRepoAppReference {
    pub repo_name: String,
    pub app_name: String,
}

impl AppRepoAppReference {
    pub fn repo_ref(&self) -> AppRepoReference {
        AppRepoReference {
            repo_name: self.repo_name.clone(),
        }
    }
}
