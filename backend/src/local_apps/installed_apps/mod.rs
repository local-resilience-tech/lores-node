use serde::Deserialize;
use utoipa::ToSchema;

pub mod fs;

#[derive(Deserialize, ToSchema, Debug)]
pub struct AppReference {
    pub app_name: String,
}
