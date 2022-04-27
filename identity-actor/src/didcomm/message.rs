// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::actor::AsyncActorRequest;
use crate::actor::Endpoint;

use super::thread_id::ThreadId;

/// A DIDComm Plaintext Message. Implementation is currently rudimentary.
///
/// See also: <https://identity.foundation/didcomm-messaging/spec/#plaintext-message-structure>.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DidCommPlaintextMessage<T> {
  pub(crate) typ: String,
  pub(crate) id: ThreadId,
  pub(crate) thid: Option<ThreadId>,
  pub(crate) pthid: Option<ThreadId>,
  #[serde(rename = "type")]
  pub(crate) type_: String,
  pub(crate) from: String,
  pub(crate) to: String,
  pub(crate) created_time: u32,
  pub(crate) expires_time: u32,
  pub(crate) body: T,
}

// TODO: Require T: DIDCommMessage and use DIDCommMessage::TYPE for validation.
impl<T> DidCommPlaintextMessage<T> {
  pub(crate) fn new(id: ThreadId, type_: String, body: T) -> Self {
    DidCommPlaintextMessage {
      id,
      type_,
      body,
      typ: String::new(),
      thid: None,
      pthid: None,
      from: String::new(),
      to: String::new(),
      created_time: 0,
      expires_time: 0,
    }
  }

  pub(crate) fn thread_id(&self) -> &ThreadId {
    match self.thid.as_ref() {
      Some(thid) => thid,
      None => &self.id,
    }
  }
}

impl<T> AsyncActorRequest for DidCommPlaintextMessage<T>
where
  T: AsyncActorRequest,
{
  fn endpoint() -> Endpoint {
    T::endpoint()
  }
}

pub trait DIDCommMessage {
  const TYPE: &'static str;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[repr(transparent)]
pub struct EmptyMessage(serde_json::Map<String, serde_json::Value>);

impl EmptyMessage {
  pub fn new() -> Self {
    Self(serde_json::Map::new())
  }
}

impl Default for EmptyMessage {
  fn default() -> Self {
    Self::new()
  }
}

impl DIDCommMessage for EmptyMessage {
  const TYPE: &'static str = "https://didcomm.org/reserved/2.0/empty";
}

impl AsyncActorRequest for EmptyMessage {
  fn endpoint() -> Endpoint {
    "didcomm/empty".parse().unwrap()
  }
}
