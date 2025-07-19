use crate::projections::entities::NodeDetails;

pub enum ClientEvent {
    NodeUpdated(NodeDetails),
}
