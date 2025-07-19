use serde::Serialize;

use crate::projections::entities::NodeDetails;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
pub enum ClientEvent {
    NodeUpdated(NodeDetails),
}
