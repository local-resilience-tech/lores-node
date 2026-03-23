use serde::Serialize;
use utoipa::ToSchema;

use crate::data::entities::{
    Region, RegionAppWithInstallations, RegionNodeDetails, RegionWithNodes,
};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum ClientEvent {
    NodeJoinedRegion(RegionWithNodes),
    RegionNodeUpdated(RegionNodeDetails),
    RegionAppUpdated(RegionAppWithInstallations),
    RegionUpdated(Region),
}
