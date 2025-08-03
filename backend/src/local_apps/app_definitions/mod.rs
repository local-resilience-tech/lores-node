use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod config;
pub mod fs;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AppDefinition {
    pub name: String,
    pub version: String,
}
