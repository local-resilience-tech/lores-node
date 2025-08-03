use serde::{Deserialize, Serialize};
use sqlx;
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct Node {
    pub id: String,
    pub name: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct NodeDetails {
    pub id: String,
    pub name: String,
    pub public_ipv4: Option<String>,
    pub status_text: Option<String>,
    pub state: Option<String>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct NodeConfig {
    pub id: String,
    pub this_node_id: String,
    pub name: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct Region {
    pub network_id: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub enum LocalAppInstallStatus {
    Installed,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct LocalApp {
    pub name: String,
    pub version: String,
    pub status: LocalAppInstallStatus,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct RegionApp {
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct RegionAppWithInstallations {
    pub name: String,
    pub installations: Vec<AppInstallation>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AppInstallation {
    pub app_name: String,
    pub node_id: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Stack {
    pub name: String,
}
