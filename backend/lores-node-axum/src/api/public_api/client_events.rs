use serde::Serialize;
use utoipa::ToSchema;

use crate::data::entities::{
    LocalApp, Region, RegionAppWithInstallations, RegionNodeDetails, RegionWithNodes,
};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum ClientEvent {
    NodeJoinedRegion(RegionWithNodes),
    RegionNodeUpdated(RegionNodeDetails),
    RegionAppUpdated(RegionAppWithInstallations),
    RegionUpdated(Region),
    LocalAppCreated(LocalApp),
    LocalAppUpdated(LocalApp),
}
