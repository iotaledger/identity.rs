// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::message::Timing;
use did_doc::url::Url;
use identity_iota::did::DID;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Trustping {
  callback_url: Url,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  context: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  thread: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl Trustping {
  pub fn new(callback_url: Url) -> Self {
    Self {
      callback_url,
      response_requested: None,
      context: None,
      id: None,
      thread: None,
      timing: None,
    }
  }

  /// Get a reference to the trustping's callback url.
  pub fn callback_url(&self) -> &Url {
    &self.callback_url
  }

  /// Get a mutable reference to the trustping's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Url {
    &mut self.callback_url
  }

  /// Set the trustping's callback url.
  pub fn set_callback_url(&mut self, callback_url: Url) {
    self.callback_url = callback_url;
  }

  /// Get a mutable reference to the trustping's response requested.
  pub fn response_requested_mut(&mut self) -> &mut Option<bool> {
    &mut self.response_requested
  }

  /// Get a reference to the trustping's response requested.
  pub fn response_requested(&self) -> &Option<bool> {
    &self.response_requested
  }

  /// Set the trustping's response requested.
  pub fn set_response_requested(&mut self, response_requested: Option<bool>) {
    self.response_requested = response_requested;
  }

  /// Get a mutable reference to the trustping's context.
  pub fn context_mut(&mut self) -> &mut Option<Url> {
    &mut self.context
  }

  /// Get a reference to the trustping's context.
  pub fn context(&self) -> &Option<Url> {
    &self.context
  }

  /// Set the trustping's context.
  pub fn set_context(&mut self, context: Option<Url>) {
    self.context = context;
  }

  /// Get a mutable reference to the trustping's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the trustping's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the trustping's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the trustping's thread.
  pub fn thread_mut(&mut self) -> &mut Option<String> {
    &mut self.thread
  }

  /// Get a reference to the trustping's thread.
  pub fn thread(&self) -> &Option<String> {
    &self.thread
  }

  /// Set the trustping's thread.
  pub fn set_thread(&mut self, thread: Option<String>) {
    self.thread = thread;
  }

  /// Get a mutable reference to the trustping's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the trustping's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the trustping's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TrustpingResponse {
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  thread: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl TrustpingResponse {
  pub fn new() -> Self {
    Self {
      id: None,
      thread: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the trustping response's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the trustping response's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the trustping response's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the trustping response's thread.
  pub fn thread_mut(&mut self) -> &mut Option<String> {
    &mut self.thread
  }

  /// Get a reference to the trustping response's thread.
  pub fn thread(&self) -> &Option<String> {
    &self.thread
  }

  /// Set the trustping response's thread.
  pub fn set_thread(&mut self, thread: Option<String>) {
    self.thread = thread;
  }

  /// Get a mutable reference to the trustping response's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the trustping response's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the trustping response's timing.
  pub fn set_timing(&mut self, timing: Option<Timing>) {
    self.timing = timing;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::envelope::SignatureAlgorithm;
  use crate::message::message::Message;
  use identity_core::crypto::KeyPair;

  #[test]
  pub fn test_plaintext_roundtrip() {
    let mut message = Trustping::new(Url::parse("https://example.com").unwrap());
    message.set_response_requested(Some(true));
    let plain_envelope = message.pack_plain().unwrap();

    let tp: Trustping = plain_envelope.to_message().unwrap();
    assert_eq!(format!("{:?}", tp), format!("{:?}", message));
  }
  #[test]
  pub fn test_signed_roundtrip() {
    let keypair = KeyPair::new_ed25519().unwrap();

    let message = Trustping::new(Url::parse("https://example.com").unwrap());
    let signed = message
      .pack_non_repudiable(SignatureAlgorithm::EdDSA, &keypair)
      .unwrap();

    let tp = signed
      .to_message::<Trustping>(SignatureAlgorithm::EdDSA, &keypair.public())
      .unwrap();
    assert_eq!(format!("{:?}", tp), format!("{:?}", message));
  }
}
