// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::actor::Endpoint;
use crate::didcomm::DidCommRequest;
use crate::didcomm::ThreadId;

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

impl<T> DidCommRequest for DidCommPlaintextMessage<T>
where
  T: DidCommRequest,
{
  fn endpoint() -> Endpoint {
    T::endpoint()
  }
}
