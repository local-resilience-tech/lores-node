use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    local_apps::app_repos::AppRepo,
    projections::entities::{LocalApp, NodeDetails, RegionAppWithInstallations},
};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum ClientEvent {
    NodeUpdated(NodeDetails),
    RegionAppUpdated(RegionAppWithInstallations),
    AppRepoUpdated(AppRepo),
    LocalAppUpdated(LocalApp),
}
