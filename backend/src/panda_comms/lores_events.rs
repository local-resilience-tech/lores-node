#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug)]
pub struct NodeAnnouncedDataV1 {
    pub name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug)]
pub struct NodeUpdatedDataV1 {
    pub name: String,
    pub public_ipv4: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug)]
pub struct NodeStatusPostedDataV1 {
    pub text: Option<String>,
    pub state: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug)]
pub enum LoResEventPayload {
    NodeAnnounced(NodeAnnouncedDataV1),
    NodeUpdated(NodeUpdatedDataV1),
    NodeStatusPosted(NodeStatusPostedDataV1),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug)]
pub enum DeprecatedLoResEventPayload {}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug)]
pub enum LoResWireEvent {
    LoResEventPayload(LoResEventPayload),
    DeprecatedLoResEventPayload(DeprecatedLoResEventPayload),
}

#[derive(Debug)]
pub struct LoResEventHeader {
    pub author_node_id: String,
}

#[derive(Debug)]
pub struct LoResEvent {
    pub header: LoResEventHeader,
    pub payload: LoResEventPayload,
}
