use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    data::entities::{NodeDetails, RegionAppWithInstallations},
    local_apps::app_repos::AppRepo,
};

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum ClientEvent {
    NodeUpdated(NodeDetails),
    RegionAppUpdated(RegionAppWithInstallations),
    AppRepoUpdated(AppRepo),
}
