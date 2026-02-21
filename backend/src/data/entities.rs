use serde::{Deserialize, Serialize};
use sqlx;
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct RegionNode {
    pub id: String,
    pub name: String,
    pub public_ipv4: Option<String>,
    pub domain_on_local_network: Option<String>,
    pub domain_on_internet: Option<String>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct RegionNodeDetails {
    pub id: String,
    pub name: String,
    pub public_ipv4: Option<String>,
    pub domain_on_local_network: Option<String>,
    pub domain_on_internet: Option<String>,
    pub status_text: Option<String>,
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Region {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct NodeAppUrl {
    pub internet_url: Option<String>,
    pub local_network_url: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct LocalApp {
    pub name: String,
    pub version: String,
    pub url: Option<NodeAppUrl>,
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
pub struct NetworkNode {
    pub id: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Network {
    pub name: String,
    pub node: NetworkNode,
}
