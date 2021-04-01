// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::message::Timing;
use did_doc::{url::Url, Document};
use identity_iota::did::DID;

#[derive(Debug, Deserialize, Serialize)]
pub struct ResolutionRequest {
  callback_url: Url,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  thread: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl ResolutionRequest {
  pub fn new(callback_url: Url) -> Self {
    Self {
      callback_url,
      id: None,
      thread: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the resolution request's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Url {
    &mut self.callback_url
  }

  /// Get a reference to the resolution request's callback url.
  pub fn callback_url(&self) -> &Url {
    &self.callback_url
  }

  /// Set the resolution request's callback url.
  pub fn set_callback_url(&mut self, callback_url: Url) {
    self.callback_url = callback_url;
  }

  /// Get a mutable reference to the resolution request's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the resolution request's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the resolution request's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the resolution request's thread.
  pub fn thread_mut(&mut self) -> &mut Option<String> {
    &mut self.thread
  }

  /// Get a reference to the resolution request's thread.
  pub fn thread(&self) -> &Option<String> {
    &self.thread
  }

  /// Set the resolution request's thread.
  pub fn set_thread(&mut self, thread: Option<String>) {
    self.thread = thread;
  }

  /// Get a mutable reference to the resolution request's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the resolution request's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the resolution request's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResolutionResponse {
  did_document: Document,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  thread: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl ResolutionResponse {
  pub fn new(did_document: Document) -> Self {
    Self {
      did_document,
      id: None,
      thread: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the resolution result's did document.
  pub fn did_document_mut(&mut self) -> &mut Document {
    &mut self.did_document
  }

  /// Get a reference to the resolution result's did document.
  pub fn did_document(&self) -> &Document {
    &self.did_document
  }

  /// Set the resolution result's did document.
  pub fn set_did_document(&mut self, did_document: Document) {
    self.did_document = did_document;
  }

  /// Get a mutable reference to the resolution result's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the resolution result's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the resolution result's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the resolution result's thread.
  pub fn thread_mut(&mut self) -> &mut Option<String> {
    &mut self.thread
  }

  /// Get a reference to the resolution result's thread.
  pub fn thread(&self) -> &Option<String> {
    &self.thread
  }

  /// Set the resolution result's thread.
  pub fn set_thread(&mut self, thread: Option<String>) {
    self.thread = thread;
  }

  /// Get a mutable reference to the resolution result's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the resolution result's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the resolution result's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}
