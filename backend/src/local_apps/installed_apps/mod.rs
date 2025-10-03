use serde::Deserialize;
use utoipa::ToSchema;

pub mod app_folder;
mod apps_folder;
pub mod config_schema;
pub mod fs;

#[derive(Deserialize, ToSchema, Debug, Clone)]
pub struct AppReference {
    pub app_name: String,
}

impl AppReference {
    pub fn new(app_name: String) -> Self {
        Self { app_name }
    }
}
