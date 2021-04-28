// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::message::Timing;
use identity_core::common::Url;
use identity_iota::did::DID;
use uuid::Uuid;

/// A DIDComm Trustping Message
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/Interactions%20and%20Messages.md#trust-ping)
///

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Trustping {
  context: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  thread: Option<Uuid>,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl Trustping {
  pub fn new(context: String) -> Self {
    Self {
      context,
      callback_url: None,
      response_requested: None,
      id: None,
      thread: None,
      timing: None,
    }
  }

  /// Get a reference to the trustping's callback url.
  pub fn callback_url(&self) -> &Option<Url> {
    &self.callback_url
  }

  /// Get a mutable reference to the trustping's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Option<Url> {
    &mut self.callback_url
  }

  /// Set the trustping's callback url.
  pub fn set_callback_url(&mut self, callback_url: Option<Url>) {
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
  pub fn context_mut(&mut self) -> &mut String {
    &mut self.context
  }

  /// Get a reference to the trustping's context.
  pub fn context(&self) -> &String {
    &self.context
  }

  /// Set the trustping's context.
  pub fn set_context(&mut self, context: String) {
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
  pub fn thread_mut(&mut self) -> &mut Option<Uuid> {
    &mut self.thread
  }

  /// Get a reference to the trustping's thread.
  pub fn thread(&self) -> &Option<Uuid> {
    &self.thread
  }

  /// Set the trustping's thread.
  pub fn set_thread(&mut self, thread: Option<Uuid>) {
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
    let ping = Trustping::new("trust-ping/1.0/ping".to_string());

    let plain_envelope_ping = ping.pack_plain().unwrap();

    let tp: Trustping = plain_envelope_ping.unpack().unwrap();

    assert_eq!(format!("{:?}", tp), format!("{:?}", ping));
  }

  #[test]
  pub fn test_signed_roundtrip() {
    let keypair = KeyPair::new_ed25519().unwrap();

    let ping = Trustping::new("trust-ping/1.0/ping".to_string());

    let signed_envelope_ping = ping.pack_non_repudiable(SignatureAlgorithm::EdDSA, &keypair).unwrap();

    let tp = signed_envelope_ping
      .unpack::<Trustping>(SignatureAlgorithm::EdDSA, &keypair.public())
      .unwrap();

    assert_eq!(format!("{:?}", tp), format!("{:?}", ping));
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

    let ping = Trustping::new("trust-ping/1.0/ping".to_string());

    let recipients = slice::from_ref(key_alice.public());

    let encoded_envelope_ping: Encrypted = ping
      .pack_auth(EncryptionAlgorithm::A256GCM, recipients, &key_bob)
      .unwrap();

    let tp: Trustping = encoded_envelope_ping
      .unpack(EncryptionAlgorithm::A256GCM, key_alice.secret(), key_bob.public())
      .unwrap();

    assert_eq!(format!("{:?}", tp), format!("{:?}", ping));
  }
}
