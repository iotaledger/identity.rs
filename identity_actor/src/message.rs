use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use identity_comm::did_comm::DIDComm;

use crate::error::IdentityMessageError;
use crate::message_type::MessageType;
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
    /// Auth Message Response
    AuthMessageResponse,
    /// An error occurred.
    // Error(IdentityMessageError),
    Panic(String),
}
