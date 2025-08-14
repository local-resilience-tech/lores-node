use serde::Deserialize;
use utoipa::ToSchema;

pub mod app_folder;
pub mod fs;

#[derive(Deserialize, ToSchema, Debug, Clone)]
pub struct AppReference {
    pub app_name: String,
}
