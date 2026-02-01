use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod stack_apps;

#[derive(Deserialize, Serialize, ToSchema, Debug, Clone)]
pub struct AppReference {
    pub app_name: String,
}
