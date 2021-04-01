// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use did_doc::{url::Url, Document, Signature};

use identity_core::crypto::{KeyPair, PublicKey};
use identity_iota::did::DID;
use serde::Serialize;

use crate::{
  envelope::{Encrypted, EncryptionAlgorithm, Plaintext, SignatureAlgorithm, Signed},
  error::Result,
  message::Timing
};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CustomMessage {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(super) callback_url: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(super) response_requested: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(super) context: Option<Url>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(super) id: Option<DID>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(super) did_document: Option<Document>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(super) thread: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(super) challenge: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(super) signature: Option<Signature>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub(super) timing: Option<Timing>,
}
pub trait Message: Serialize {}


pub trait AsEnvelope {
  fn pack_plain(&self) -> Result<Plaintext>;
  fn pack_auth(&self, algorithm: EncryptionAlgorithm, recipients: &[PublicKey], sender: &KeyPair) -> Result<Encrypted>;
  fn pack_auth_non_repudiable(
    &self,
    signature: SignatureAlgorithm,
    encryption: EncryptionAlgorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Encrypted>;
  fn pack_anon(&self, algorithm: EncryptionAlgorithm, recipients: &[PublicKey]) -> Result<Encrypted>;
  fn pack_non_repudiable(&self, algorithm: SignatureAlgorithm, sender: &KeyPair) -> Result<Signed>;
}

impl<T: Serialize> AsEnvelope for T {
  fn pack_plain(&self) -> Result<Plaintext> {
    Plaintext::from_message(self)
  }

  fn pack_auth(&self, algorithm: EncryptionAlgorithm, recipients: &[PublicKey], sender: &KeyPair) -> Result<Encrypted> {
    Encrypted::from_message(self, algorithm, recipients, sender)
  }

  fn pack_auth_non_repudiable(
    &self,
    signature: SignatureAlgorithm,
    encryption: EncryptionAlgorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Encrypted> {
    Self::pack_non_repudiable(self, signature, sender)
      .and_then(|signed| Encrypted::from_signed(&signed, encryption, recipients, sender))
  }

  fn pack_anon(&self, algorithm: EncryptionAlgorithm, recipients: &[PublicKey]) -> Result<Encrypted> {
    Encrypted::anon_from_message(self, algorithm, recipients)
  }

  fn pack_non_repudiable(&self, algorithm: SignatureAlgorithm, sender: &KeyPair) -> Result<Signed> {
    Signed::from_message(self, algorithm, sender)
  }
}
