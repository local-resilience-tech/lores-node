use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod git;
pub mod installed;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AppRepoSource {
    pub name: String,
    pub git_url: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AppRepo {
    pub name: String,
}
