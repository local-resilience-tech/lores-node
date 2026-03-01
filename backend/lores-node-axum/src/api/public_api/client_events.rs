use serde::Serialize;
use utoipa::ToSchema;

use crate::data::entities::{RegionAppWithInstallations, RegionNodeDetails, RegionWithNodes};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum ClientEvent {
    JoinedRegion(RegionWithNodes),
    RegionNodeUpdated(RegionNodeDetails),
    RegionAppUpdated(RegionAppWithInstallations),
}
