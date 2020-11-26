use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use identity_comm::did_comm::DIDComm;

use crate::error::IdentityMessageError;
use serde::{ser::Serializer, Deserialize, Serialize};

/// The message type.
#[derive(Debug, Clone)]
pub struct Message {
    id: usize,
    pub(crate) message_type: MessageType,
    pub(crate) response_tx: UnboundedSender<Response>,
}

impl Message {
    /// Creates a new instance of a Message.
    pub fn new(id: usize, message_type: MessageType, response_tx: UnboundedSender<Response>) -> Self {
        Self {
            id,
            message_type,
            response_tx,
        }
    }

    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }

    pub fn response_tx(&self) -> &UnboundedSender<Response> {
        &self.response_tx
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// The messages that can be sent to the actor.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "cmd", content = "payload")]
pub enum MessageType {
    /// Remove the account related to the specified `account_id`.
    TrustPing,
}

impl Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MessageType::TrustPing => serializer.serialize_unit_variant("MessageType", 0, "TrustPing"),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Response {
    id: usize,
    #[serde(flatten)]
    response: ResponseType,
    action: MessageType,
}

impl Response {
    pub fn new(id: usize, action: MessageType, response: ResponseType) -> Self {
        Self { id, response, action }
    }

    pub fn response(&self) -> &ResponseType {
        &self.response
    }
}

/// The response message.
#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "payload")]
pub enum ResponseType {
    /// Trust Ping Response
    TrustPingResponse,
    /// An error occurred.
    // Error(IdentityMessageError),
    Panic(String),
}
