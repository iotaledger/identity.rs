// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use crate::envelope::Encrypted;
use crate::envelope::EncryptionAlgorithm;
use crate::envelope::Plaintext;
use crate::envelope::SignatureAlgorithm;
use crate::envelope::Signed;
use crate::error::Result;
use identity_core::crypto::KeyPair;
use identity_core::crypto::PublicKey;
use serde::Serialize;

pub trait Message {
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

impl<T: Serialize> Message for T {
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
