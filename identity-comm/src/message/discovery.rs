// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::message::Timing;
use identity_core::common::Url;
use identity_iota::did::DID;

/// A DIDComm Did Discovery Message
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/Interactions%20and%20Messages.md#did-discovery)
///
#[derive(Debug, Deserialize, Serialize)]
pub struct DidRequest {
  callback_url: Url,
  #[serde(skip_serializing_if = "Option::is_none")]
  context: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  thread: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl DidRequest {
  pub fn new(callback_url: Url) -> Self {
    Self {
      callback_url,
      context: None,
      id: None,
      thread: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the did request's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Url {
    &mut self.callback_url
  }

  /// Get a reference to the did request's callback url.
  pub fn callback_url(&self) -> &Url {
    &self.callback_url
  }

  /// Set the did request's callback url.
  pub fn set_callback_url(&mut self, callback_url: Url) {
    self.callback_url = callback_url;
  }

  /// Get a mutable reference to the did request's context.
  pub fn context_mut(&mut self) -> &mut Option<Url> {
    &mut self.context
  }

  /// Get a reference to the did request's context.
  pub fn context(&self) -> &Option<Url> {
    &self.context
  }

  /// Set the did request's context.
  pub fn set_context(&mut self, context: Option<Url>) {
    self.context = context;
  }

  /// Get a mutable reference to the did request's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the did request's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the did request's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the did request's thread.
  pub fn thread_mut(&mut self) -> &mut Option<String> {
    &mut self.thread
  }

  /// Get a reference to the did request's thread.
  pub fn thread(&self) -> &Option<String> {
    &self.thread
  }

  /// Set the did request's thread.
  pub fn set_thread(&mut self, thread: Option<String>) {
    self.thread = thread;
  }

  /// Get a mutable reference to the did request's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the did request's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the did request's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DidResponse {
  id: DID,
}

impl DidResponse {
  pub fn new(id: DID) -> Self {
    Self { id }
  }

  /// Get a mutable reference to the did response's id.
  pub fn id_mut(&mut self) -> &mut DID {
    &mut self.id
  }

  /// Get a reference to the did response's id.
  pub fn id(&self) -> &DID {
    &self.id
  }

  /// Set the did response's id.
  pub fn set_id(&mut self, id: DID) {
    self.id = id;
  }
}
