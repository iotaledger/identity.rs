use crate::{did_comm::DIDComm, messages::MessageType};
use identity_core::{common::Timestamp, did::DID};
use serde::{Deserialize, Serialize};

/// A `DIDCommBuilder` is used to create a customized `DIDComm`.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DIDCommBuilder {
    pub(crate) id: String, // MUST be unique to the sender
    #[serde(rename = "type")]
    pub(crate) comm_type: MessageType, // MUST be a valid Message Type URI
    pub(crate) from: Option<String>, // MUST be a string that is a valid DID which identifies the sender of the message
    pub(crate) to: Option<Vec<String>>, // MUST be an array of strings where each element is a valid DID
    pub(crate) created_at: Option<Timestamp>, /* expressed in UTC Epoch Seconds (example: 1599692536) -> https://github.com/iotaledger/identity.rs/blob/952e8c86ff58954b15bb2a4964dfb7b6fe39b122/identity_core/src/common/timestamp.rs */
    pub(crate) expires_at: Option<Timestamp>, /* expressed in UTC Epoch Seconds (example: 1599692536) -> https://github.com/iotaledger/identity.rs/blob/952e8c86ff58954b15bb2a4964dfb7b6fe39b122/identity_core/src/common/timestamp.rs */
    pub(crate) body: Option<String>,          // Here can be everything
}

impl DIDCommBuilder {
    /// Initializes the DIDComm struct with the filled out fields.
    pub fn new() -> Self {
        Self {
            id: String::new(),
            comm_type: MessageType::TrustPing,
            from: None,
            to: None,
            created_at: None,
            expires_at: None,
            body: None,
        }
    }

    /// Sets the `id` value.
    #[must_use]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the `comm_type` value.
    #[must_use]
    pub fn comm_type(mut self, message_type: MessageType) -> Self {
        self.comm_type = message_type;
        self
    }
    /// Sets the `from` value.
    #[must_use]
    pub fn from(mut self, did: DID) -> Self {
        self.from = Some(did.to_string());
        self
    }
    /// Sets the `to` value.
    #[must_use]
    pub fn to(mut self, dids: Vec<DID>) -> Self {
        self.to = Some(dids.iter().map(|d| d.to_string()).collect());
        self
    }
    /// Sets the `created_at` value.
    #[must_use]
    pub fn created_at(mut self, time: Timestamp) -> Self {
        self.created_at = Some(time);
        self
    }
    /// Sets the `expires_at` value.
    #[must_use]
    pub fn expires_at(mut self, time: Timestamp) -> Self {
        self.expires_at = Some(time);
        self
    }
    /// Sets the `body` value.
    #[must_use]
    pub fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    /// Returns a new `DIDComm` based on the `DIDCommBuilder` configuration.
    pub fn build(self) -> crate::Result<DIDComm> {
        DIDComm::from_builder(self)
    }
}

impl Default for DIDCommBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_didcomm_builder() {
    println!("start");
    let didcomm: DIDComm = DIDCommBuilder::default()
        .id("unique id")
        .comm_type(MessageType::TrustPing)
        .build()
        .unwrap();
    println!("didcomm{:?}", didcomm);
}
