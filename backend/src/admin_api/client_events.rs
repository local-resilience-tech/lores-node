use serde::Serialize;
use utoipa::ToSchema;

use crate::projections::entities::{NodeDetails, RegionAppWithInstallations};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum ClientEvent {
    NodeUpdated(NodeDetails),
    RegionAppUpdated(RegionAppWithInstallations),
}
