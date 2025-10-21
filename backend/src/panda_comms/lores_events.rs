use p2panda_core::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct NodeAnnouncedDataV1 {
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct NodeUpdatedDataV1 {
    pub name: String,
    pub public_ipv4: String,
    pub domain_on_local_network: Option<String>,
    pub domain_on_internet: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct NodeStatusPostedDataV1 {
    pub text: Option<String>,
    pub state: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct AppRegisteredDataV1 {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum LoResEventPayload {
    NodeAnnounced(NodeAnnouncedDataV1),
    NodeUpdated(NodeUpdatedDataV1),
    NodeStatusPosted(NodeStatusPostedDataV1),
    AppRegistered(AppRegisteredDataV1),
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum DeprecatedLoResEventPayload {}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub enum LoResPossibleEventPayload {
    LoResEventPayload(LoResEventPayload),
    DeprecatedLoResEventPayload(DeprecatedLoResEventPayload),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LoResEventMetadataV1 {
    pub node_steward_id: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LoResWirePayload {
    pub metadata: LoResEventMetadataV1,
    pub event_payload: LoResPossibleEventPayload,
}

#[derive(Debug, Clone)]
pub struct LoResEventHeader {
    pub author_node_id: String,

    // Time in milliseconds since the Unix epoch
    pub timestamp: u64,

    pub operation_id: Hash,
}

#[derive(Debug, Clone)]
pub struct LoResEvent {
    pub header: LoResEventHeader,
    pub payload: LoResEventPayload,
}
