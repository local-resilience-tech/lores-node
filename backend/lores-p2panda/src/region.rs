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

/// Derive a p2panda topic from a region and an application namespace.
///
/// The region ID is the base, and the app namespace further shards it so that
/// different applications sharing the same region do not receive each other's
/// operations.
///
/// ```text
/// topic = Hash(region_id_bytes || app_namespace_utf8)
/// ```
pub fn derive_topic(region_id: &RegionId, app_namespace: &str) -> Topic {
    let mut data = Vec::with_capacity(32 + app_namespace.len());
    data.extend_from_slice(&region_id.bytes);
    data.extend_from_slice(app_namespace.as_bytes());
    Topic::from(*Hash::digest(&data).as_bytes())
}
