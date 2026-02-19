use serde::Serialize;
use utoipa::ToSchema;

use crate::data::entities::{RegionAppWithInstallations, RegionNodeDetails};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum ClientEvent {
    NodeUpdated(RegionNodeDetails),
    RegionAppUpdated(RegionAppWithInstallations),
}
