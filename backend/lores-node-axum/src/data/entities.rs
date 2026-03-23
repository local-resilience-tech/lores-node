use lores_p2panda::p2panda_core::PublicKey;
use serde::{Deserialize, Serialize};
use sqlx;
use utoipa::ToSchema;

use crate::panda_comms::RegionId;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, PartialEq, sqlx::Type)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

impl LatLng {
    pub fn validate(&self) -> Result<(), String> {
        if self.lat < -90.0 || self.lat > 90.0 {
            return Err(format!(
                "Invalid latitude: {}. Must be between -90 and 90.",
                self.lat
            ));
        }
        if self.lng < -180.0 || self.lng > 180.0 {
            return Err(format!(
                "Invalid longitude: {}. Must be between -180 and 180.",
                self.lng
            ));
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, ToSchema, sqlx::Type, Debug, Clone)]
pub enum RegionNodeStatus {
    RequestedToJoin,
    Member,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct RegionNode {
    pub id: i64,
    pub node_id: String,
    pub region_id: String,
    pub status: Option<RegionNodeStatus>,
    pub name: Option<String>,
    pub public_ipv4: Option<String>,
    pub domain_on_local_network: Option<String>,
    pub domain_on_internet: Option<String>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct RegionNodeDetails {
    pub id: i64,
    pub node_id: String,
    pub region_id: String,
    pub status: Option<RegionNodeStatus>,
    pub name: Option<String>,
    pub public_ipv4: Option<String>,
    pub domain_on_local_network: Option<String>,
    pub domain_on_internet: Option<String>,
    pub about_your_node: Option<String>,
    pub about_your_stewards: Option<String>,
    pub agreed_node_steward_conduct_url: Option<String>,
    pub status_text: Option<String>,
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct Region {
    pub id: String,
    pub creator_node_id: Option<String>,
    pub slug: Option<String>,
    pub name: Option<String>,
    pub organisation_name: Option<String>,
    pub organisation_url: Option<String>,
    pub node_steward_conduct_url: Option<String>,
    pub user_conduct_url: Option<String>,
    pub user_privacy_url: Option<String>,
    pub map: Option<String>,
    pub min_latlng: Option<LatLng>,
    pub max_latlng: Option<LatLng>,
}

impl Region {
    pub fn unnamed(id: RegionId, creator_node_id: PublicKey) -> Self {
        let hex_id = id.to_hex();

        Self {
            id: hex_id.clone(),
            creator_node_id: Some(creator_node_id.to_string()),
            slug: Some(hex_id.chars().take(12).collect()),
            name: Some(hex_id.chars().take(12).collect()),
            organisation_name: None,
            organisation_url: None,
            node_steward_conduct_url: None,
            user_conduct_url: None,
            user_privacy_url: None,
            map: None,
            min_latlng: None,
            max_latlng: None,
        }
    }
}

#[derive(Serialize, ToSchema, Debug, Clone)]
pub struct RegionWithNodes {
    pub region: Region,
    pub nodes: Vec<RegionNodeDetails>,
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
pub struct RegionAppWithInstallations {
    pub name: String,
    pub installations: Vec<AppInstallation>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
pub struct AppInstallation {
    pub app_name: String,
    pub region_node_id: i64,
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
