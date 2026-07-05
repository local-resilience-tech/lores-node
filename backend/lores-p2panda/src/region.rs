use std::fmt;

use hex::FromHexError;
use p2panda_core::{Hash, Topic};

/// A 32-byte identifier for a region in the lores network.
///
/// At the p2panda level a region is just a scoped namespace — the lores
/// application layer gives it richer meaning (member nodes, metadata, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegionId {
    bytes: [u8; 32],
}

impl RegionId {
    pub fn from_hex(value: &str) -> Result<RegionId, FromHexError> {
        let mut bytes = [0u8; 32];
        hex::decode_to_slice(value, &mut bytes as &mut [u8])?;
        Ok(RegionId { bytes })
    }

    pub fn generate() -> Self {
        let mut bytes = [0u8; 32];
        rand::fill(&mut bytes[..]);
        RegionId { bytes }
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.bytes)
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.bytes
    }
}

impl From<RegionId> for [u8; 32] {
    fn from(id: RegionId) -> Self {
        id.bytes
    }
}

impl From<[u8; 32]> for RegionId {
    fn from(bytes: [u8; 32]) -> Self {
        Self { bytes }
    }
}

impl fmt::Display for RegionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

pub trait RegionTopic {
    fn p2panda_topic(&self) -> Topic;
}

#[derive(Clone)]
pub struct RegionAdminTopic {
    pub region_id: RegionId,
}

impl RegionAdminTopic {
    pub fn new(region_id: RegionId) -> Self {
        Self { region_id }
    }
}

impl RegionTopic for RegionAdminTopic {
    fn p2panda_topic(&self) -> Topic {
        Topic::from(<[u8; 32]>::from(self.region_id.clone()))
    }
}

#[derive(Clone)]
pub struct RegionAppTopic {
    pub region_id: RegionId,
    pub app_id: String,
}

impl RegionAppTopic {
    pub fn new(region_id: RegionId, app_id: impl Into<String>) -> Self {
        Self {
            region_id,
            app_id: app_id.into(),
        }
    }
}

impl RegionTopic for RegionAppTopic {
    fn p2panda_topic(&self) -> Topic {
        let mut data = Vec::with_capacity(32 + self.app_id.len());
        data.extend_from_slice(&self.region_id.bytes);
        data.extend_from_slice(self.app_id.as_bytes());
        Topic::from(*Hash::digest(&data).as_bytes())
    }
}
