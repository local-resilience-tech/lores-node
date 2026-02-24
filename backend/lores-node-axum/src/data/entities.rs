use lores_p2panda::p2panda_core::PublicKey;
use serde::{Deserialize, Serialize};
use sqlx;
use utoipa::ToSchema;

use crate::panda_comms::RegionId;

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

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Region {
    pub id: String,
    pub creator_node_id: String,
    pub slug: String,
    pub name: String,
    pub organisation_name: Option<String>,
    pub url: Option<String>,
}

impl Region {
    pub fn unnamed(id: RegionId, creator_node_id: PublicKey) -> Self {
        let hex_id = id.to_hex();

        Self {
            id: hex_id.clone(),
            creator_node_id: creator_node_id.to_string(),
            slug: hex_id.chars().take(12).collect(),
            name: hex_id.chars().take(12).collect(),
            organisation_name: None,
            url: None,
        }
    }
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
