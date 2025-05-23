#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NodeAnnounced {
    pub name: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NodeUpdated {
    pub name: String,
    pub public_ipv4: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum LoResEventPayload {
    NodeAnnounced(NodeAnnounced),
    NodeUpdated(NodeUpdated),
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
