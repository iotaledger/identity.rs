// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::envelope::EnvelopeExt;
use crate::envelope::Plaintext;
use crate::error::Result;
use identity_core::crypto::KeyPair;
use identity_core::crypto::PublicKey;
use identity_core::utils::encode_b58;
use libjose::jose::JoseTokenType;
use libjose::jws::Decoder;
use libjose::jws::Encoder;
use libjose::jws::JwsAlgorithm;
use libjose::jws::JwsFormat;
use libjose::jws::JwsHeader;
use serde::Deserialize;
use serde::Serialize;

/// Supported digital signature algorithms
///
/// [Reference](https://identity.foundation/didcomm-messaging/spec/#algorithms)
#[derive(Clone, Copy, Debug)]
pub enum Algorithm {
  EdDSA, // crv=Ed25519
  ES256,
  ES256K,
}

impl From<Algorithm> for JwsAlgorithm {
  fn from(other: Algorithm) -> Self {
    match other {
      Algorithm::EdDSA => Self::EdDSA,
      Algorithm::ES256 => Self::ES256,
      Algorithm::ES256K => Self::ES256K,
    }
  }
}

/// A DIDComm Signed Message
///
/// [Reference](https://identity.foundation/didcomm-messaging/spec/#didcomm-signed-message)
///
/// # Layout
///
///   `JWS(Plaintext)`
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Envelope(String);

impl Envelope {
  pub fn from_message<T: Serialize>(message: &T, algorithm: Algorithm, keypair: &KeyPair) -> Result<Self> {
    Plaintext::from_message(message).and_then(|plaintext| Self::from_plaintext(&plaintext, algorithm, keypair))
  }

  pub fn from_plaintext(envelope: &Plaintext, algorithm: Algorithm, keypair: &KeyPair) -> Result<Self> {
    let header: JwsHeader = {
      let mut header: JwsHeader = JwsHeader::new(algorithm.into());
      header.set_kid(encode_b58(keypair.public()));
      header.set_typ(JoseTokenType::JWM.name());
      header
    };

    Encoder::new()
      .format(JwsFormat::Compact)
      .recipient((keypair.secret().as_ref(), &header))
      .encode(envelope.as_bytes())
      .map_err(Into::into)
      .map(Self)
  }

  pub fn to_message<T>(&self, algorithm: Algorithm, public: &PublicKey) -> Result<T>
  where
    for<'a> T: Deserialize<'a>,
  {
    let token = Decoder::new(public.as_ref())
      .key_id(encode_b58(public))
      .format(JwsFormat::Compact)
      .algorithm(algorithm.into())
      .decode(self.as_bytes())?;

    serde_json::from_slice(&token.claims.to_vec()).map_err(Into::into)
  }
}

impl EnvelopeExt for Envelope {
  const FEXT: &'static str = "dcsm";
  const MIME: &'static str = "application/didcomm-signed+json";

  fn as_bytes(&self) -> &[u8] {
    self.0.as_bytes()
  }
}
