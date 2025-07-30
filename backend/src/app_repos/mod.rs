use serde::Deserialize;
use utoipa::ToSchema;

pub mod git;

#[derive(Deserialize, ToSchema)]
pub struct AppRepoSource {
    pub name: String,
    pub git_url: String,
}
