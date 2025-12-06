use serde::Serialize;
use utoipa::ToSchema;

use crate::data::entities::{NodeDetails, RegionAppWithInstallations};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum ClientEvent {
    NodeUpdated(NodeDetails),
    RegionAppUpdated(RegionAppWithInstallations),
}
