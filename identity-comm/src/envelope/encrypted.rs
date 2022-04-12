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
pub struct DidCommEncryptedMessage(pub String);

impl DidCommEncryptedMessage {
  pub fn pack<T: ToJson>(
    message: &T,
    cek_algorithm: CEKAlgorithm,
    enc_algorithm: EncryptionAlgorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Self> {
    Plaintext::pack(message)
      .and_then(|plaintext| Self::pack_plaintext(&plaintext, cek_algorithm, enc_algorithm, recipients, sender))
  }

  pub fn pack_plaintext(
    envelope: &Plaintext,
    cek_algorithm: CEKAlgorithm,
    enc_algorithm: EncryptionAlgorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Self> {
    Self::pack_envelope(envelope, cek_algorithm, enc_algorithm, recipients, sender)
  }

  pub fn pack_signed(
    envelope: &Signed,
    cek_algorithm: CEKAlgorithm,
    enc_algorithm: EncryptionAlgorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Self> {
    Self::pack_envelope(envelope, cek_algorithm, enc_algorithm, recipients, sender)
  }

  fn pack_envelope<T: EnvelopeExt>(
    envelope: &T,
    cek_algorithm: CEKAlgorithm,
    enc_algorithm: EncryptionAlgorithm,
    recipients: &[PublicKey],
    sender: &KeyPair,
  ) -> Result<Self> {
    let header: JweHeader = JweHeader::new(JweAlgorithm::from(cek_algorithm), enc_algorithm.into());

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
    cek_algorithm: CEKAlgorithm,
    enc_algorithm: EncryptionAlgorithm,
    recipient: &PrivateKey,
    sender: &PublicKey,
  ) -> Result<T> {
    let bytes: Vec<u8> = self.unpack_vec(cek_algorithm, enc_algorithm, recipient, sender)?;

    T::from_json_slice(&bytes).map_err(Into::into)
  }

  pub fn unpack_vec(
    &self,
    cek_algorithm: CEKAlgorithm,
    enc_algorithm: EncryptionAlgorithm,
    recipient: &PrivateKey,
    sender: &PublicKey,
  ) -> Result<Vec<u8>> {
    let token: Token = Decoder::new(recipient)
      .public(sender)
      .format(JweFormat::General)
      .algorithm(JweAlgorithm::from(cek_algorithm))
      .encryption(enc_algorithm.into())
      .decode(self.as_bytes())?;

    Ok(token.1)
  }
}

impl EnvelopeExt for DidCommEncryptedMessage {
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

/// Supported algorithms for the cryptographic algorithm used to encrypt
/// or determine the value of the CEK.
#[derive(Clone, Copy, Debug)]
pub enum CEKAlgorithm {
  /// Can be used for sender-authenticated encryption.
  ECDH_1PU_A256KW,
  /// Can be used for anonymous encryption.
  ECDH_ES_A256KW,
}

impl From<CEKAlgorithm> for JweAlgorithm {
  fn from(other: CEKAlgorithm) -> Self {
    match other {
      CEKAlgorithm::ECDH_1PU_A256KW => JweAlgorithm::ECDH_1PU_A256KW,
      CEKAlgorithm::ECDH_ES_A256KW => JweAlgorithm::ECDH_ES_A256KW,
    }
  }
}
