// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;
use core::marker::PhantomData;
use crypto::signatures::ed25519;
use crypto::signatures::ed25519::PUBLIC_KEY_LENGTH;
use crypto::signatures::ed25519::SECRET_KEY_LENGTH;
use crypto::signatures::ed25519::SIGNATURE_LENGTH;

use crate::crypto::key::KeyError;
use crate::crypto::Sign;
use crate::crypto::Verify;

use self::errors::SignatureParsingError;
use super::errors::ProofValueError;
use super::errors::SigningError;
use super::errors::VerificationError;
use super::errors::VerificationProcessingError;

/// An implementation of `Ed25519` signatures.
#[derive(Clone, Copy, Debug)]
pub struct Ed25519<T: ?Sized = [u8]>(PhantomData<T>);

impl<T> Sign for Ed25519<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  type Private = T;
  type Output = [u8; SIGNATURE_LENGTH];
  /// Computes an EdDSA/Ed25519 signature.
  ///
  /// The private key must be a 32-byte seed in compliance with [RFC 8032](https://datatracker.ietf.org/doc/html/rfc8032#section-3.2).
  /// Other implementations often use another format. See [this blog post](https://blog.mozilla.org/warner/2011/11/29/ed25519-keys/) for further explanation.
  fn sign(message: &[u8], key: &Self::Private) -> Result<Self::Output, SigningError> {
    parse_secret(key.as_ref())
      .map(|key| key.sign(message).to_bytes())
      .map_err(Into::into)
  }
}

impl<T> Verify for Ed25519<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  type Public = T;

  fn verify(message: &[u8], signature: &[u8], key: &Self::Public) -> Result<(), VerificationError> {
    let key: ed25519::PublicKey = parse_public(key.as_ref())?;
    let sig: ed25519::Signature = parse_signature(signature).map_err(|err| VerificationProcessingError::from(err.0))?;

    if key.verify(&sig, message) {
      Ok(())
    } else {
      Err(ProofValueError("ed25519").into())
    }
  }
}

fn parse_public(slice: &[u8]) -> Result<ed25519::PublicKey, KeyError> {
  let bytes: [u8; PUBLIC_KEY_LENGTH] = slice
    .try_into()
    .map_err(|_| KeyError("could not create a public key from the supplied bytes: incorrect length"))?;
  ed25519::PublicKey::try_from_bytes(bytes).map_err(|_| KeyError("could not parse public key from the supplied bytes"))
}

fn parse_secret(slice: &[u8]) -> Result<ed25519::SecretKey, KeyError> {
  let bytes: [u8; SECRET_KEY_LENGTH] = slice
    .try_into()
    .map_err(|_| KeyError("could not create a secret key from the supplied bytes: incorrect length"))?;

  Ok(ed25519::SecretKey::from_bytes(bytes))
}

fn parse_signature(slice: &[u8]) -> Result<ed25519::Signature, SignatureParsingError> {
  let bytes: [u8; SIGNATURE_LENGTH] = slice
    .try_into()
    .map_err(|_| SignatureParsingError("could not parse a signature from the supplied bytes"))?;
  Ok(ed25519::Signature::from_bytes(bytes))
}

mod errors {
  use thiserror::Error as DeriveError;
  #[derive(Debug, DeriveError)]
  /// Caused by a failure to parse a signature
  #[error("{0}")]
  pub(super) struct SignatureParsingError(pub(super) &'static str);
}
#[cfg(test)]
mod tests {
  use crate::crypto::Ed25519;
  use crate::crypto::Sign;
  use crate::crypto::Verify;

  // The following test vector is taken from [Test 3 of RFC 8032](https://datatracker.ietf.org/doc/html/rfc8032#section-7)
  const PUBLIC_KEY_HEX: &str = "fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025";
  const SECRET_KEY_HEX: &str = "c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7";
  const MESSAGE_HEX: &str = "af82";
  const SIGNATURE_HEX: &str = "6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a";

  #[test]
  fn test_ed25519_can_sign_and_verify() {
    let public_key = hex::decode(PUBLIC_KEY_HEX).unwrap();
    let private_key = hex::decode(SECRET_KEY_HEX).unwrap();
    let message = hex::decode(MESSAGE_HEX).unwrap();
    let signature = Ed25519::sign(&message, &private_key).unwrap();
    assert_eq!(&hex::encode(signature), SIGNATURE_HEX);
    let verified: _ = Ed25519::verify(&hex::decode(MESSAGE_HEX).unwrap()[..], &signature, &public_key);
    assert!(verified.is_ok());
  }
}
