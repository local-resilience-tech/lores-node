use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod git;
pub mod installed;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AppRepoSource {
    pub name: String,
    pub git_url: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AppDefinition {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct AppRepo {
    pub name: String,
    pub apps: Vec<AppDefinition>,
}

// pub struct RepoAppReference {
//     pub repo_name: String,
//     pub app_name: String,
// }
