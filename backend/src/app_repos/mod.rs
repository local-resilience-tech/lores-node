use serde::Deserialize;
use utoipa::ToSchema;

pub mod git;

#[derive(Deserialize, ToSchema)]
pub struct AppRepo {
    pub name: String,
    pub git_url: String,
}
