use p2panda_core::{Hash, PublicKey, Topic};

/// An operation received from the network (or replayed from local storage) for a subscribed topic.
pub struct IncomingOperation {
    pub author: PublicKey,
    pub topic: Topic,
    pub bytes: Vec<u8>,
    pub operation_id: Hash,
    pub timestamp: u64,
}

pub use crate::panda_node::SubscriptionError;
