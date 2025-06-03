use rocket::serde::{Deserialize, Serialize};
use sqlx;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Node {
    pub id: String,
    pub name: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NodeDetails {
    pub id: String,
    pub name: String,
    pub public_ipv4: Option<String>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NodeConfig {
    pub id: String,
    pub this_node_id: String,
    pub name: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Region {
    pub network_id: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PrivateKeyRow {
    pub private_key_hex: Option<String>,
}

pub struct NodeStatus {
    pub operation_id: String,
    pub author_node_id: String,
    pub posted_timestamp: u64,
    pub text: Option<String>,
    pub state: Option<String>,
}
