// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::message::Timing;
use identity_core::common::Url;
use identity_iota::did::DID;
use uuid::Uuid;

/// A DIDComm did-discovery message
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/i_did-discovery.md)
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DidRequest {
  context: String,
  thread: Uuid,
  callback_url: Url,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl DidRequest {
  pub fn new(context: String, thread: Uuid, callback_url: Url) -> Self {
    Self {
      context,
      thread,
      callback_url,
      response_requested: None,
      id: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the did request's context.
  pub fn context_mut(&mut self) -> &mut String {
    &mut self.context
  }

  /// Get a reference to the did request's context.
  pub fn context(&self) -> &String {
    &self.context
  }

  /// Set the did request's context.
  pub fn set_context(&mut self, context: String) {
    self.context = context;
  }

  /// Get a mutable reference to the did request's thread.
  pub fn thread_mut(&mut self) -> &mut Uuid {
    &mut self.thread
  }

  /// Get a reference to the did request's thread.
  pub fn thread(&self) -> &Uuid {
    &self.thread
  }

  /// Set the did request's thread.
  pub fn set_thread(&mut self, thread: Uuid) {
    self.thread = thread;
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

  /// Get a mutable reference to the did request's response requested.
  pub fn response_requested_mut(&mut self) -> &mut Option<bool> {
    &mut self.response_requested
  }

  /// Get a reference to the did request's response requested.
  pub fn response_requested(&self) -> &Option<bool> {
    &self.response_requested
  }

  /// Set the did request's response requested.
  pub fn set_response_requested(&mut self, response_requested: Option<bool>) {
    self.response_requested = response_requested;
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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DidResponse {
  context: String,
  thread: Uuid,
  id: DID,
  #[serde(skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl DidResponse {
  pub fn new(context: String, thread: Uuid, id: DID) -> Self {
    Self {
      context,
      thread,
      id,
      callback_url: None,
      response_requested: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the did response's context.
  pub fn context_mut(&mut self) -> &mut String {
    &mut self.context
  }

  /// Get a reference to the did response's context.
  pub fn context(&self) -> &String {
    &self.context
  }

  /// Set the did response's context.
  pub fn set_context(&mut self, context: String) {
    self.context = context;
  }

  /// Get a mutable reference to the did response's thread.
  pub fn thread_mut(&mut self) -> &mut Uuid {
    &mut self.thread
  }

  /// Get a reference to the did response's thread.
  pub fn thread(&self) -> &Uuid {
    &self.thread
  }

  /// Set the did response's thread.
  pub fn set_thread(&mut self, thread: Uuid) {
    self.thread = thread;
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

  /// Get a mutable reference to the did response's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Option<Url> {
    &mut self.callback_url
  }

  /// Get a reference to the did response's callback url.
  pub fn callback_url(&self) -> &Option<Url> {
    &self.callback_url
  }

  /// Set the did response's callback url.
  pub fn set_callback_url(&mut self, callback_url: Option<Url>) {
    self.callback_url = callback_url;
  }

  /// Get a mutable reference to the did response's response requested.
  pub fn response_requested_mut(&mut self) -> &mut Option<bool> {
    &mut self.response_requested
  }

  /// Get a reference to the did response's response requested.
  pub fn response_requested(&self) -> &Option<bool> {
    &self.response_requested
  }

  /// Set the did response's response requested.
  pub fn set_response_requested(&mut self, response_requested: Option<bool>) {
    self.response_requested = response_requested;
  }

  /// Get a mutable reference to the did response's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the did response's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the did response's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}
