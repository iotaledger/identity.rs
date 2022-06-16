// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
//! Functionality for creating [signed DIDComm messages](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message)

use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::KeyPair;
use identity_core::crypto::PublicKey;
use identity_core::utils::encode_b58;
use libjose::jose::JoseTokenType;
use libjose::jws::Decoder;
use libjose::jws::Encoder;
use libjose::jws::JwsAlgorithm;
use libjose::jws::JwsFormat;
use libjose::jws::JwsHeader;

use crate::envelope::EnvelopeExt;
use crate::envelope::Plaintext;
use crate::error::Result;

/// A DIDComm Signed Message
///
/// [Reference](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message)
///
/// # Layout
///
///   `JWS(Plaintext)`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Signed(pub(crate) String);

impl Signed {
  pub fn pack_plaintext(message: &Plaintext, algorithm: SignatureAlgorithm, keypair: &KeyPair) -> Result<Self> {
    let header: JwsHeader = {
      let mut header: JwsHeader = JwsHeader::new(algorithm.into());
      header.set_kid(encode_b58(keypair.public()));
      header.set_typ(JoseTokenType::JWM.name());
      header
    };

    Encoder::new()
      .format(JwsFormat::Compact)
      .recipient((keypair.private(), &header))
      .encode(message.as_bytes())
      .map_err(Into::into)
      .map(Self)
  }

  pub fn unpack_plaintext(&self, algorithm: SignatureAlgorithm, public: &PublicKey) -> Result<Plaintext> {
    let claims: Vec<u8> = Decoder::new(public)
      .key_id(encode_b58(public))
      .format(JwsFormat::Compact)
      .algorithm(algorithm.into())
      .decode(self.0.as_bytes())
      .map(|token| token.claims.to_vec())?;

    Ok(Plaintext(String::from_utf8(claims)?))
  }

  pub fn pack<T: ToJson>(message: &T, algorithm: SignatureAlgorithm, keypair: &KeyPair) -> Result<Self> {
    Plaintext::pack(message).and_then(|plaintext| Self::pack_plaintext(&plaintext, algorithm, keypair))
  }

  pub fn unpack<T: FromJson>(&self, algorithm: SignatureAlgorithm, public: &PublicKey) -> Result<T> {
    self
      .unpack_plaintext(algorithm, public)
      .and_then(|plaintext| plaintext.unpack())
  }
}

impl EnvelopeExt for Signed {
  const FEXT: &'static str = "dcsm";
  const MIME: &'static str = "application/didcomm-signed+json";

  fn as_bytes(&self) -> &[u8] {
    self.0.as_bytes()
  }
}

// =============================================================================
// =============================================================================

/// Supported digital signature algorithms
///
/// [Reference](https://identity.foundation/didcomm-messaging/spec/#algorithms)
#[derive(Clone, Copy, Debug)]
pub enum SignatureAlgorithm {
  EdDSA, // crv=Ed25519
  ES256,
  ES256K,
}

impl From<SignatureAlgorithm> for JwsAlgorithm {
  fn from(other: SignatureAlgorithm) -> Self {
    match other {
      SignatureAlgorithm::EdDSA => Self::EdDSA,
      SignatureAlgorithm::ES256 => Self::ES256,
      SignatureAlgorithm::ES256K => Self::ES256K,
    }
  }
}
