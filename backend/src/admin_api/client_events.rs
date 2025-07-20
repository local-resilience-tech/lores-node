use serde::Serialize;
use utoipa::ToSchema;

use crate::projections::entities::NodeDetails;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum ClientEvent {
    NodeUpdated(NodeDetails),
}
