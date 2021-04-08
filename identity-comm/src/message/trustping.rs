// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::message::Timing;
use identity_core::common::Url;
use identity_iota::did::DID;

/// A DIDComm Trustping Message
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/Interactions%20and%20Messages.md#trust-ping)
///

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
  use core::slice;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::PublicKey;
  use identity_core::crypto::SecretKey;
  use libjose::utils::ed25519_to_x25519_public;
  use libjose::utils::ed25519_to_x25519_secret;

  use super::*;
  use crate::envelope::Encrypted;
  use crate::envelope::EncryptionAlgorithm;
  use crate::envelope::SignatureAlgorithm;
  use crate::error::Result;
  use crate::message::Message;

  #[test]
  pub fn test_plaintext_roundtrip() {
    let ping = Trustping::new(Url::parse("https://example.com").unwrap());
    let pong = TrustpingResponse::new();

    let plain_envelope_ping = ping.pack_plain().unwrap();
    let plain_envelope_pong = pong.pack_plain().unwrap();

    let tp: Trustping = plain_envelope_ping.unpack().unwrap();
    let tpr: TrustpingResponse = plain_envelope_pong.unpack().unwrap();

    assert_eq!(format!("{:?}", tp), format!("{:?}", ping));
    assert_eq!(format!("{:?}", tpr), format!("{:?}", pong));
  }

  #[test]
  pub fn test_signed_roundtrip() {
    let keypair = KeyPair::new_ed25519().unwrap();

    let ping = Trustping::new(Url::parse("https://example.com").unwrap());
    let pong = TrustpingResponse::new();

    let signed_envelope_ping = ping.pack_non_repudiable(SignatureAlgorithm::EdDSA, &keypair).unwrap();
    let singed_envelope_pong = ping.pack_non_repudiable(SignatureAlgorithm::EdDSA, &keypair).unwrap();

    let tp = signed_envelope_ping
      .unpack::<Trustping>(SignatureAlgorithm::EdDSA, &keypair.public())
      .unwrap();
    let tpr = singed_envelope_pong
      .unpack::<TrustpingResponse>(SignatureAlgorithm::EdDSA, &keypair.public())
      .unwrap();

    assert_eq!(format!("{:?}", tp), format!("{:?}", ping));
    assert_eq!(format!("{:?}", tpr), format!("{:?}", pong));
  }

  fn ed25519_to_x25519(keypair: KeyPair) -> Result<(PublicKey, SecretKey)> {
    Ok((
      ed25519_to_x25519_public(keypair.public())?.to_vec().into(),
      ed25519_to_x25519_secret(keypair.secret())?.to_vec().into(),
    ))
  }

  fn ed25519_to_x25519_keypair(keypair: KeyPair) -> Result<KeyPair> {
    // This is completely wrong but `type_` is never used around here
    let type_ = keypair.type_();
    let (public, secret) = ed25519_to_x25519(keypair)?;
    Ok((type_, public, secret).into())
  }

  #[test]
  pub fn test_encrypted_roundtrip() {
    let key_alice = KeyPair::new_ed25519().unwrap();
    let key_alice = ed25519_to_x25519_keypair(key_alice).unwrap();

    let key_bob = KeyPair::new_ed25519().unwrap();
    let key_bob = ed25519_to_x25519_keypair(key_bob).unwrap();

    let ping = Trustping::new(Url::parse("https://example.com").unwrap());
    let pong = TrustpingResponse::new();

    let recipients = slice::from_ref(key_alice.public());

    let encoded_envelope_ping: Encrypted = ping
      .pack_auth(EncryptionAlgorithm::A256GCM, recipients, &key_bob)
      .unwrap();
    let encoded_envelope_pong: Encrypted = pong
      .pack_auth(EncryptionAlgorithm::A256GCM, recipients, &key_bob)
      .unwrap();

    let tp: Trustping = encoded_envelope_ping
      .unpack(EncryptionAlgorithm::A256GCM, key_alice.secret(), key_bob.public())
      .unwrap();
    let tpr: TrustpingResponse = encoded_envelope_pong
      .unpack(EncryptionAlgorithm::A256GCM, key_alice.secret(), key_bob.public())
      .unwrap();

    assert_eq!(format!("{:?}", tp), format!("{:?}", ping));
    assert_eq!(format!("{:?}", tpr), format!("{:?}", pong));
  }
}
