// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Defines how to pack messages into envelopes.
use identity_core::convert::ToJson;
use identity_core::crypto::KeyPair;
use identity_core::crypto::PublicKey;

use crate::envelope::Encrypted;
use crate::envelope::EncryptionAlgorithm;
use crate::envelope::Plaintext;
use crate::envelope::SignatureAlgorithm;
use crate::envelope::Signed;
use crate::error::Result;

/// A general-purpose extension to pack messages into envelopes.
pub trait Message {
  fn pack_plain(&self) -> Result<Plaintext>;

  fn pack_signed(&self, algorithm: SignatureAlgorithm, sender: &KeyPair) -> Result<Signed>;

  fn pack_encrypted(
    &self,
    cek_algorithm: CEKAlgorithm,
    enc_algorithm: EncryptionAlgorithm,
    recipients: &[(PublicKey, String)],
    sender: &KeyPair,
  ) -> Result<Encrypted>;

  fn pack_signed_encrypted(
    &self,
    signature: SignatureAlgorithm,
    encryption: EncryptionAlgorithm,
    recipients: &[(PublicKey, String)],
    sender: &KeyPair,
  ) -> Result<Encrypted>;
}

impl<T: ToJson> Message for T {
  fn pack_plain(&self) -> Result<Plaintext> {
    Plaintext::pack(self)
  }

  fn pack_signed(&self, algorithm: SignatureAlgorithm, sender: &KeyPair) -> Result<Signed> {
    Signed::pack(self, algorithm, sender)
  }

  fn pack_encrypted(
    &self,
    cek_algorithm: CEKAlgorithm,
    enc_algorithm: EncryptionAlgorithm,
    recipients: &[(PublicKey, String)],
    sender: &KeyPair,
  ) -> Result<Encrypted> {
    Encrypted::pack(self, algorithm, recipients, sender)
  }

  fn pack_signed_encrypted(
    &self,
    signature: SignatureAlgorithm,
    encryption: EncryptionAlgorithm,
    recipients: &[PublicKey],
    cek_algorithm: CEKAlgorithm,
  ) -> Result<Encrypted> {
    Self::pack_signed(self, signature, sender)
      .and_then(|signed| Encrypted::pack_signed(&signed, encryption, recipients, sender))
  }
}