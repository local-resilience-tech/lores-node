#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug)]
pub struct NodeAnnouncedData {
    pub name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug)]
pub struct NodeUpdatedData {
    pub name: String,
    pub public_ipv4: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Debug)]
pub enum LoResEventPayload {
    NodeAnnounced(NodeAnnouncedData),
    NodeUpdated(NodeUpdatedData),
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
