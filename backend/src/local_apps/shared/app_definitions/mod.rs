use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod config;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AppVersionDefinition {
    pub name: String,
    pub version: String,
}
