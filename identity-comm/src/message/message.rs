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

/// A Message Trait to pack messages into Envelopes

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

  fn pack_non_repudiable(&self, algorithm: SignatureAlgorithm, sender: &KeyPair) -> Result<Signed>;
}

impl<T: Clone + std::fmt::Debug + Send + Sync + 'static + Serialize> Message for T {
  fn pack_plain(&self) -> Result<Plaintext> {
    Plaintext::pack(self)
  }

  fn pack_non_repudiable(&self, algorithm: SignatureAlgorithm, sender: &KeyPair) -> Result<Signed> {
    Signed::pack(self, algorithm, sender)
  }

  fn pack_auth(&self, algorithm: EncryptionAlgorithm, recipients: &[PublicKey], sender: &KeyPair) -> Result<Encrypted> {
    Encrypted::pack(self, algorithm, recipients, sender)
  }

  fn pack_auth_non_repudiable(
    &self,
    signature: SignatureAlgorithm,
    encryption: EncryptionAlgorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Encrypted> {
    Self::pack_non_repudiable(self, signature, sender)
      .and_then(|signed| Encrypted::pack_signed(&signed, encryption, recipients, sender))
  }
}
