use crate::did_comm_builder::DIDCommBuilder;
use identity_core::common::Timestamp;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::messages::MessageType;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DIDComm {
    pub id: String, // MUST be unique to the sender
    #[serde(rename = "type")]
    pub comm_type: MessageType, // MUST be a valid Message Type URI
    pub from: Option<String>, // MUST be a string that is a valid DID which identifies the sender of the message
    pub to: Option<Vec<String>>, // MUST be an array of strings where each element is a valid DID
    pub created_at: Option<Timestamp>, /* expressed in UTC Epoch Seconds (example: 1599692536) -> https://github.com/iotaledger/identity.rs/blob/952e8c86ff58954b15bb2a4964dfb7b6fe39b122/identity_core/src/common/timestamp.rs */
    pub expires_at: Option<Timestamp>, /* expressed in UTC Epoch Seconds (example: 1599692536) -> https://github.com/iotaledger/identity.rs/blob/952e8c86ff58954b15bb2a4964dfb7b6fe39b122/identity_core/src/common/timestamp.rs */
    pub body: Option<String>,          // Here can be everything
}

impl DIDComm {
    /// Returns a new `DIDComm` based on the `DIDCommBuilder` configuration.
    pub fn from_builder(builder: DIDCommBuilder) -> crate::Result<Self> {
        let this: Self = Self {
            id: builder.id,
            comm_type: builder.comm_type,
            from: builder.from,
            to: builder.to,
            created_at: builder.created_at,
            expires_at: builder.expires_at,
            body: builder.body,
        };
        Ok(this)
    }
}

/// converts a `DIDComm` Message into a string using the `to_string()` method.
impl ToString for DIDComm {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).expect("Unable to serialize DIDComm Message")
    }
}

/// takes a &str and converts it into a `DIDComm` Message given the proper format.
impl FromStr for DIDComm {
    type Err = crate::Error;

    fn from_str(s: &str) -> crate::Result<Self> {
        let did_comm = serde_json::from_str(s).expect("Unable to build DIDComm Message");
        Ok(did_comm)
    }
}
