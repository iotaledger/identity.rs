use serde::{Deserialize, Serialize};

use crate::messages::trust_ping::TRUSTPING;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum MessageType {
    TrustPing,
}
