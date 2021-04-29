// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::message::Timing;
use identity_core::common::Url;
use identity_iota::did::DID;
use uuid::Uuid;

/// A DIDComm features-discovery message
///
/// [Reference](https://github.com/iotaledger/identity.rs/blob/dev/docs/DID%20Communications%20Research%20and%20Specification/i_features-discovery.md)
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FeaturesRequest {
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

impl FeaturesRequest {
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
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FeaturesResponse {
  context: String,
  thread: Uuid,
  features: Vec<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  callback_url: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  timing: Option<Timing>,
}

impl FeaturesResponse {
  pub fn new(context: String, thread: Uuid, features: Vec<String>) -> Self {
    Self {
      context,
      thread,
      features,
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

  /// Get a mutable reference to the features response's features.
  pub fn features_mut(&mut self) -> &mut Vec<String> {
    &mut self.features
  }

  /// Get a reference to the features response's features.
  pub fn features(&self) -> &Vec<String> {
    &self.features
  }

  /// Set the features response's features.
  pub fn set_features(&mut self, features: Vec<String>) {
    self.features = features;
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
    let did_request = FeaturesRequest::new(
      "did-discovery/1.0/didRequest".to_string(),
      Uuid::new_v4(),
      Url::parse("https://example.com").unwrap(),
    );
    let did_response = FeaturesResponse::new(
      "did-discovery/1.0/didResponse".to_string(),
      Uuid::new_v4(),
      vec!["trust-ping/1.0".to_string(), "did-discovery/1.0".to_string()],
    );

    let plain_envelope_request = did_request.pack_plain().unwrap();
    let plain_envelope_response = did_response.pack_plain().unwrap();

    let request: FeaturesRequest = plain_envelope_request.unpack().unwrap();
    let response: FeaturesResponse = plain_envelope_response.unpack().unwrap();

    assert_eq!(format!("{:?}", request), format!("{:?}", did_request));
    assert_eq!(format!("{:?}", response), format!("{:?}", did_response));
  }

  #[test]
  pub fn test_signed_roundtrip() {
    let keypair = KeyPair::new_ed25519().unwrap();

    let did_request = FeaturesRequest::new(
      "did-discovery/1.0/didRequest".to_string(),
      Uuid::new_v4(),
      Url::parse("https://example.com").unwrap(),
    );
    let did_response = FeaturesResponse::new(
      "did-discovery/1.0/didResponse".to_string(),
      Uuid::new_v4(),
      vec!["trust-ping/1.0".to_string(), "did-discovery/1.0".to_string()],
    );
    let signed_request = did_request
      .pack_non_repudiable(SignatureAlgorithm::EdDSA, &keypair)
      .unwrap();

    let signed_response = did_response
      .pack_non_repudiable(SignatureAlgorithm::EdDSA, &keypair)
      .unwrap();

    let request = signed_request
      .unpack::<FeaturesRequest>(SignatureAlgorithm::EdDSA, &keypair.public())
      .unwrap();

    let response = signed_response
      .unpack::<FeaturesResponse>(SignatureAlgorithm::EdDSA, &keypair.public())
      .unwrap();

    assert_eq!(format!("{:?}", request), format!("{:?}", did_request));
    assert_eq!(format!("{:?}", response), format!("{:?}", did_response));
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

    let did_request = FeaturesRequest::new(
      "did-discovery/1.0/didRequest".to_string(),
      Uuid::new_v4(),
      Url::parse("https://example.com").unwrap(),
    );
    let did_response = FeaturesResponse::new(
      "did-discovery/1.0/didResponse".to_string(),
      Uuid::new_v4(),
      vec!["trust-ping/1.0".to_string(), "did-discovery/1.0".to_string()],
    );
    let recipients = slice::from_ref(key_alice.public());

    let encoded_request: Encrypted = did_request
      .pack_auth(EncryptionAlgorithm::A256GCM, recipients, &key_bob)
      .unwrap();

    let encoded_response: Encrypted = did_response
      .pack_auth(EncryptionAlgorithm::A256GCM, recipients, &key_bob)
      .unwrap();

    let decoded_request: FeaturesRequest = encoded_request
      .unpack(EncryptionAlgorithm::A256GCM, key_alice.secret(), key_bob.public())
      .unwrap();

    let decoded_response: FeaturesResponse = encoded_response
      .unpack(EncryptionAlgorithm::A256GCM, key_alice.secret(), key_bob.public())
      .unwrap();

    assert_eq!(format!("{:?}", decoded_request), format!("{:?}", did_request));
    assert_eq!(format!("{:?}", decoded_response), format!("{:?}", did_response));
  }
}
