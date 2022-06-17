// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Functionality for creating [DIDComm encrypted messages](https://identity.foundation/didcomm-messaging/spec/#didcomm-encrypted-message)

#![allow(non_camel_case_types)]

use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::KeyPair;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;
use libjose::jwe::Decoder;
use libjose::jwe::Encoder;
use libjose::jwe::JweAlgorithm;
use libjose::jwe::JweEncryption;
use libjose::jwe::JweFormat;
use libjose::jwe::JweHeader;
use libjose::jwe::Token;

use crate::envelope::EnvelopeExt;
use crate::envelope::Plaintext;
use crate::envelope::Signed;
use crate::error::Result;

/// A DIDComm Encrypted Message
///
/// [Reference](https://identity.foundation/didcomm-messaging/spec/#didcomm-encrypted-message)
///
/// # Layout
///
///   `JWE(Plaintext | Signed)`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Encrypted(pub(crate) String);

impl Encrypted {
  pub fn pack<T: ToJson>(
    message: &T,
    algorithm: EncryptionAlgorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Self> {
    Plaintext::pack(message).and_then(|plaintext| Self::pack_plaintext(&plaintext, algorithm, recipients, sender))
  }

  pub fn pack_plaintext(
    envelope: &Plaintext,
    algorithm: EncryptionAlgorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Self> {
    Self::pack_envelope(envelope, algorithm, recipients, sender)
  }

  pub fn pack_signed(
    envelope: &Signed,
    algorithm: EncryptionAlgorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Self> {
    Self::pack_envelope(envelope, algorithm, recipients, sender)
  }

  fn pack_envelope<T: EnvelopeExt>(
    envelope: &T,
    algorithm: EncryptionAlgorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Self> {
    let header: JweHeader = JweHeader::new(JweAlgorithm::ECDH_1PU, algorithm.into());

    let encoder: Encoder<'_> = Encoder::new()
      .format(JweFormat::General)
      .protected(&header)
      .secret(sender.private());

    recipients
      .iter()
      .fold(encoder, |encoder, recipient| encoder.recipient(recipient))
      .encode(envelope.as_bytes())
      .map_err(Into::into)
      .map(Self)
  }

  pub fn unpack<T: FromJson>(
    &self,
    algorithm: EncryptionAlgorithm,
    recipient: &PrivateKey,
    sender: &PublicKey,
  ) -> Result<T> {
    let token: Token = Decoder::new(recipient)
      .public(sender)
      .format(JweFormat::General)
      .algorithm(JweAlgorithm::ECDH_1PU)
      .encryption(algorithm.into())
      .decode(self.as_bytes())?;

    T::from_json_slice(&token.1).map_err(Into::into)
  }
}

impl EnvelopeExt for Encrypted {
  const FEXT: &'static str = "dcem";
  const MIME: &'static str = "application/didcomm-encrypted+json";

  fn as_bytes(&self) -> &[u8] {
    self.0.as_bytes()
  }
}

// =============================================================================
// =============================================================================

/// Supported content encryption algorithms
///
/// [Reference (auth)](https://identity.foundation/didcomm-messaging/spec/#sender-authenticated-encryption)
/// [Reference (anon)](https://identity.foundation/didcomm-messaging/spec/#anonymous-encryption)
#[derive(Clone, Copy, Debug)]
pub enum EncryptionAlgorithm {
  A256GCM,
  XC20P,
}

impl From<EncryptionAlgorithm> for JweEncryption {
  fn from(other: EncryptionAlgorithm) -> Self {
    match other {
      EncryptionAlgorithm::A256GCM => Self::A256GCM,
      EncryptionAlgorithm::XC20P => Self::XC20P,
    }
  }
}
