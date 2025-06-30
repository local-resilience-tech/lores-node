use serde::{Deserialize, Serialize};
use sqlx;
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct Node {
    pub id: String,
    pub name: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
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

#[derive(sqlx::FromRow, Serialize, Deserialize, ToSchema)]
pub struct PrivateKeyRow {
    pub private_key_hex: Option<String>,
}
