// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Defines how to pack messages into envelopes.
use identity_core::convert::ToJson;
use identity_core::crypto::KeyPair;
use identity_core::crypto::PublicKey;

use crate::envelope::CEKAlgorithm;
use crate::envelope::DidCommEncryptedMessage;
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
  ) -> Result<DidCommEncryptedMessage>;

  fn pack_signed_encrypted(
    &self,
    signature: SignatureAlgorithm,
    cek_algorithm: CEKAlgorithm,
    encryption: EncryptionAlgorithm,
    recipients: &[(PublicKey, String)],
    sender: &KeyPair,
  ) -> Result<DidCommEncryptedMessage>;
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
  ) -> Result<DidCommEncryptedMessage> {
    DidCommEncryptedMessage::pack(self, cek_algorithm, enc_algorithm, recipients, sender)
  }

  fn pack_signed_encrypted(
    &self,
    signature: SignatureAlgorithm,
    cek_algorithm: CEKAlgorithm,
    enc_algorithm: EncryptionAlgorithm,
    recipients: &[(PublicKey, String)],
    sender: &KeyPair,
  ) -> Result<DidCommEncryptedMessage> {
    Self::pack_signed(self, signature, sender).and_then(|signed| {
      DidCommEncryptedMessage::pack_signed(&signed, cek_algorithm, enc_algorithm, recipients, sender)
    })
  }
}
