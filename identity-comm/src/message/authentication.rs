// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::message::Timing;
use crate::message::Message;
use did_doc::{url::Url, Signature};
use identity_iota::did::DID;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticationRequest {
  callback_url: Url,
  thread: String,
  challenge: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl AuthenticationRequest {
  pub fn new(callback_url: Url, thread: String, challenge: String) -> Self {
    Self {
      callback_url,
      thread,
      challenge,
      id: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the authentication request's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Url {
    &mut self.callback_url
  }

  /// Get a reference to the authentication request's callback url.
  pub fn callback_url(&self) -> &Url {
    &self.callback_url
  }

  /// Set the authentication request's callback url.
  pub fn set_callback_url(&mut self, callback_url: Url) {
    self.callback_url = callback_url;
  }

  /// Get a mutable reference to the authentication request's thread.
  pub fn thread_mut(&mut self) -> &mut String {
    &mut self.thread
  }

  /// Get a reference to the authentication request's thread.
  pub fn thread(&self) -> &String {
    &self.thread
  }

  /// Set the authentication request's thread.
  pub fn set_thread(&mut self, thread: String) {
    self.thread = thread;
  }

  /// Get a mutable reference to the authentication request's challenge.
  pub fn challenge_mut(&mut self) -> &mut String {
    &mut self.challenge
  }

  /// Get a reference to the authentication request's challenge.
  pub fn challenge(&self) -> &String {
    &self.challenge
  }

  /// Set the authentication request's challenge.
  pub fn set_challenge(&mut self, challenge: String) {
    self.challenge = challenge;
  }

  /// Get a mutable reference to the authentication request's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the authentication request's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the authentication request's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the authentication request's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the authentication request's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the authentication request's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticationResponse {
  thread: String,
  signature: Signature,
}

impl AuthenticationResponse {
  pub fn new(thread: String, signature: Signature) -> Self {
    Self { thread, signature }
  }

  /// Get a mutable reference to the authentication response's thread.
  pub fn thread_mut(&mut self) -> &mut String {
    &mut self.thread
  }

  /// Get a reference to the authentication response's thread.
  pub fn thread(&self) -> &String {
    &self.thread
  }

  /// Set the authentication response's thread.
  pub fn set_thread(&mut self, thread: String) {
    self.thread = thread;
  }

  /// Get a mutable reference to the authentication response's signature.
  pub fn signature_mut(&mut self) -> &mut Signature {
    &mut self.signature
  }

  /// Get a reference to the authentication response's signature.
  pub fn signature(&self) -> &Signature {
    &self.signature
  }

  /// Set the authentication response's signature.
  pub fn set_signature(&mut self, signature: Signature) {
    self.signature = signature;
  }
}

impl Message for AuthenticationRequest {}
impl Message for AuthenticationResponse {}
