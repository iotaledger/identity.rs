// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Endpoint;
use crate::Result;

use libp2p::PeerId;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMessage
// where
//   T: Serialize + for<'deser> Deserialize<'deser>,
{
  pub endpoint: Endpoint,
  pub data: Vec<u8>,
}

impl RequestMessage
// where
//   T: Serialize + for<'deser> Deserialize<'deser>,
{
  pub fn new(name: impl AsRef<str>, data: Vec<u8>) -> Result<Self> {
    Ok(Self {
      endpoint: Endpoint::new(name)?,
      data,
    })
  }
}

pub type ResponseMessage = Vec<u8>;

pub struct RequestContext<T> {
  pub input: T,
  pub peer: PeerId,
  pub endpoint: Endpoint,
}

impl<T> RequestContext<T> {
  pub fn new(input: T, peer: PeerId, endpoint: Endpoint) -> Self {
    Self { input, peer, endpoint }
  }

  pub fn convert<I>(self, input: I) -> RequestContext<I> {
    RequestContext::new(input, self.peer, self.endpoint)
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DidCommPlaintextMessage {
  pub typ: String,
  pub id: ThreadId,
  pub thid: Option<ThreadId>,
  pub pthid: Option<ThreadId>,
  #[serde(rename = "type")]
  pub type_: String,
  pub from: String,
  pub to: String,
  pub created_time: u32,
  pub expires_time: u32,
  pub body: serde_json::Value,
}

impl DidCommPlaintextMessage {
  pub fn new(id: ThreadId, type_: String, body: serde_json::Value) -> Self {
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

  pub fn thread_id(&self) -> &ThreadId {
    match self.thid.as_ref() {
      Some(thid) => thid,
      None => &self.id,
    }
  }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThreadId {
  inner: String,
}

impl ThreadId {
  pub fn new() -> Self {
    Self {
      inner: uuid::Uuid::new_v4().to_string(),
    }
  }
}

impl Display for ThreadId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.inner)
  }
}
