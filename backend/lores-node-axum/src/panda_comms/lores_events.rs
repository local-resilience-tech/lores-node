use lores_p2panda::p2panda_core::hash::Hash;
use serde::{Deserialize, Serialize};

use super::RegionId;

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct RegionCreatedDataV1 {
    pub slug: String,
    pub name: String,
    pub organisation_name: Option<String>,
    pub organisation_url: Option<String>,
    pub node_steward_conduct_url: Option<String>,
    pub user_conduct_url: Option<String>,
    pub user_privacy_url: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct RegionJoinRequestedDataV1 {
    pub about_your_node: String,
    pub about_your_stewards: String,
    pub agreed_node_steward_conduct_url: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct RegionJoinRequestApprovedDataV1 {
    pub region_id: String,
    pub node_id: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct RegionNodeUpdatedDataV1 {
    pub name: Option<String>,
    pub public_ipv4: Option<String>,
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
    RegionCreated(RegionCreatedDataV1),
    RegionJoinRequested(RegionJoinRequestedDataV1),
    RegionJoinRequestApproved(RegionJoinRequestApprovedDataV1),
    RegionNodeUpdated(RegionNodeUpdatedDataV1),
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
    pub region_id: Option<RegionId>,

    // Time in milliseconds since the Unix epoch
    pub timestamp: u64,

    pub operation_id: Hash,
}

#[derive(Debug, Clone)]
pub struct LoResEvent {
    pub header: LoResEventHeader,
    pub payload: LoResEventPayload,
}
