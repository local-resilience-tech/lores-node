use serde::Serialize;
use utoipa::ToSchema;

use crate::data::entities::{Region, RegionAppWithInstallations, RegionNodeDetails};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum ClientEvent {
    JoinedRegion(Region),
    NodeUpdated(RegionNodeDetails),
    RegionAppUpdated(RegionAppWithInstallations),
}
