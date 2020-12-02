use serde::{ser::Serializer, Deserialize, Serialize};

/// The messages that can be sent to the actor.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "cmd", content = "payload")]
pub enum MessageType {
    /// Remove the account related to the specified `account_id`.
    TrustPing,
    AuthMessage
}

impl Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MessageType::TrustPing => serializer.serialize_unit_variant("MessageType", 0, "TrustPing"),
            MessageType::AuthMessage => serializer.serialize_unit_variant("MessageType", 0, "AuthMessage"),
        }
    }
}