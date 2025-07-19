use crate::projections::entities::NodeDetails;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ClientEvent {
    NodeUpdated(NodeDetails),
}
