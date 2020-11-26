pub mod trust_ping;

use serde::{Deserialize, Serialize};

// use crate::messages::trust_ping::TRUSTPING;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum MessageType {
    /// should be TRUSTPING?
    TrustPing,
}
