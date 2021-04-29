// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::message::Timing;
use identity_core::common::Url;
use identity_core::crypto::Signature;
use identity_iota::did::DID;
use serde::Serialize;
use uuid::Uuid;
/// A DIDComm Autentication Request
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/i_authentication.md)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AuthenticationRequest {
  context: String,
  thread: Uuid,
  callback_url: Url,
  challenge: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl AuthenticationRequest {
  pub fn new(context: String, thread: Uuid, callback_url: Url, challenge: String) -> Self {
    Self {
      context,
      thread,
      callback_url,
      challenge,
      response_requested: None,
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
  pub fn thread_mut(&mut self) -> &mut Uuid {
    &mut self.thread
  }

  /// Get a reference to the authentication request's thread.
  pub fn thread(&self) -> &Uuid {
    &self.thread
  }

  /// Set the authentication request's thread.
  pub fn set_thread(&mut self, thread: Uuid) {
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

  /// Get a mutable reference to the authentication request's context.
  pub fn context_mut(&mut self) -> &mut String {
    &mut self.context
  }

  /// Get a reference to the authentication request's context.
  pub fn context(&self) -> &String {
    &self.context
  }

  /// Set the authentication request's context.
  pub fn set_context(&mut self, context: String) {
    self.context = context;
  }

  /// Get a mutable reference to the authentication request's response requested.
  pub fn response_requested_mut(&mut self) -> &mut Option<bool> {
    &mut self.response_requested
  }

  /// Get a reference to the authentication request's response requested.
  pub fn response_requested(&self) -> &Option<bool> {
    &self.response_requested
  }

  /// Set the authentication request's response requested.
  pub fn set_response_requested(&mut self, response_requested: Option<bool>) {
    self.response_requested = response_requested;
  }
}
// Todo: implement new method for signing of the whole AuthenticationRequest
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AuthenticationResponse {
  context: String,
  thread: Uuid,
  signature: Signature,
  #[serde(skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl AuthenticationResponse {
  pub fn new(context: String, thread: Uuid, signature: Signature) -> Self {
    Self {
      context,
      thread,
      signature,
      callback_url: None,
      response_requested: None,
      id: None,
      timing: None,
    }
  }

  /// Get a mutable reference to the authentication response's thread.
  pub fn thread_mut(&mut self) -> &mut Uuid {
    &mut self.thread
  }

  /// Get a reference to the authentication response's thread.
  pub fn thread(&self) -> &Uuid {
    &self.thread
  }

  /// Set the authentication response's thread.
  pub fn set_thread(&mut self, thread: Uuid) {
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

  /// Get a mutable reference to the authentication response's context.
  pub fn context_mut(&mut self) -> &mut String {
    &mut self.context
  }

  /// Get a reference to the authentication response's context.
  pub fn context(&self) -> &String {
    &self.context
  }

  /// Set the authentication response's context.
  pub fn set_context(&mut self, context: String) {
    self.context = context;
  }

  /// Get a mutable reference to the authentication response's callback url.
  pub fn callback_url_mut(&mut self) -> &mut Option<Url> {
    &mut self.callback_url
  }

  /// Get a reference to the authentication response's callback url.
  pub fn callback_url(&self) -> &Option<Url> {
    &self.callback_url
  }

  /// Set the authentication response's callback url.
  pub fn set_callback_url(&mut self, callback_url: Option<Url>) {
    self.callback_url = callback_url;
  }

  /// Get a mutable reference to the authentication response's response requested.
  pub fn response_requested_mut(&mut self) -> &mut Option<bool> {
    &mut self.response_requested
  }

  /// Get a reference to the authentication response's response requested.
  pub fn response_requested(&self) -> &Option<bool> {
    &self.response_requested
  }

  /// Set the authentication response's response requested.
  pub fn set_response_requested(&mut self, response_requested: Option<bool>) {
    self.response_requested = response_requested;
  }

  /// Get a mutable reference to the authentication response's id.
  pub fn id_mut(&mut self) -> &mut Option<DID> {
    &mut self.id
  }

  /// Get a reference to the authentication response's id.
  pub fn id(&self) -> &Option<DID> {
    &self.id
  }

  /// Set the authentication response's id.
  pub fn set_id(&mut self, id: Option<DID>) {
    self.id = id;
  }

  /// Get a mutable reference to the authentication response's timing.
  pub fn timing_mut(&mut self) -> &mut Option<Timing> {
    &mut self.timing
  }

  /// Get a reference to the authentication response's timing.
  pub fn timing(&self) -> &Option<Timing> {
    &self.timing
  }

  /// Set the authentication response's timing.
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
    let authentication_request = AuthenticationRequest::new(
      "authentication/1.0/authenticationRequest".to_string(),
      Uuid::new_v4(),
      Url::parse("htpps://example.com").unwrap(),
      "please sign this".to_string(),
    );
    let plain_envelope_request = authentication_request.pack_plain().unwrap();
    let request: AuthenticationRequest = plain_envelope_request.unpack().unwrap();
    assert_eq!(format!("{:?}", request), format!("{:?}", authentication_request));
  }

  #[test]
  pub fn test_signed_roundtrip() {
    let keypair = KeyPair::new_ed25519().unwrap();

    let authentication_request = AuthenticationRequest::new(
      "authentication/1.0/authenticationRequest".to_string(),
      Uuid::new_v4(),
      Url::parse("htpps://example.com").unwrap(),
      "please sign this".to_string(),
    );
    let signed_request = authentication_request
      .pack_non_repudiable(SignatureAlgorithm::EdDSA, &keypair)
      .unwrap();

    let request = signed_request
      .unpack::<AuthenticationRequest>(SignatureAlgorithm::EdDSA, &keypair.public())
      .unwrap();

    assert_eq!(format!("{:?}", request), format!("{:?}", authentication_request));
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

    let authentication_request = AuthenticationRequest::new(
      "authentication/1.0/authenticationRequest".to_string(),
      Uuid::new_v4(),
      Url::parse("htpps://example.com").unwrap(),
      "please sign this".to_string(),
    );
    let recipients = slice::from_ref(key_alice.public());

    let encoded_request: Encrypted = authentication_request
      .pack_auth(EncryptionAlgorithm::A256GCM, recipients, &key_bob)
      .unwrap();

    let decoded_request: AuthenticationRequest = encoded_request
      .unpack(EncryptionAlgorithm::A256GCM, key_alice.secret(), key_bob.public())
      .unwrap();

    assert_eq!(
      format!("{:?}", decoded_request),
      format!("{:?}", authentication_request)
    );
  }
}
