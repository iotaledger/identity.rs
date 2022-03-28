// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;
use core::marker::PhantomData;
use crypto::signatures::ed25519;

use crate::crypto::Sign;
use crate::crypto::Verify;
use crate::error::Error;
use crate::error::Result;

/// An implementation of `Ed25519` signatures.
#[derive(Clone, Copy, Debug)]
pub struct Ed25519<T: ?Sized = [u8]>(PhantomData<T>);

impl Ed25519 {
  /// Length in bytes of an Ed25519 private key.
  pub const PRIVATE_KEY_LENGTH: usize = ed25519::SECRET_KEY_LENGTH;
  /// Length in bytes of an Ed25519 public key.
  pub const PUBLIC_KEY_LENGTH: usize = ed25519::PUBLIC_KEY_LENGTH;
  /// Length in bytes of an Ed25519 signature.
  pub const SIGNATURE_LENGTH: usize = ed25519::SIGNATURE_LENGTH;
}

impl<T> Sign for Ed25519<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  type Private = T;
  type Output = [u8; Ed25519::SIGNATURE_LENGTH];
  /// Computes an EdDSA signature using an Ed25519 private key.
  ///
  /// The private key must be a 32-byte seed in compliance with [RFC 8032](https://datatracker.ietf.org/doc/html/rfc8032#section-3.2).
  /// Other implementations often use another format. See [this blog post](https://blog.mozilla.org/warner/2011/11/29/ed25519-keys/) for further explanation.
  fn sign(message: &[u8], key: &Self::Private) -> Result<Self::Output> {
    ed25519_private_try_from_bytes(key.as_ref()).map(|key| key.sign(message).to_bytes())
  }
}

impl<T> Verify for Ed25519<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  type Public = T;

  /// Verifies an EdDSA signature against an Ed25519 public key.
  fn verify(message: &[u8], signature: &[u8], key: &Self::Public) -> Result<()> {
    let key: ed25519::PublicKey = ed25519_public_try_from_bytes(key.as_ref())?;
    let sig: ed25519::Signature = parse_signature(signature)?;

    if key.verify(&sig, message) {
      Ok(())
    } else {
      Err(Error::InvalidProofValue("ed25519"))
    }
  }
}

fn parse_signature(slice: &[u8]) -> Result<ed25519::Signature> {
  let bytes: [u8; Ed25519::SIGNATURE_LENGTH] = slice
    .try_into()
    .map_err(|_| Error::InvalidSigLength(slice.len(), Ed25519::SIGNATURE_LENGTH))?;

  Ok(ed25519::Signature::from_bytes(bytes))
}

/// Reconstructs an Ed25519 private key from a byte array.
pub(crate) fn ed25519_private_try_from_bytes(bytes: &[u8]) -> Result<ed25519::SecretKey> {
  let private_key_bytes: [u8; Ed25519::PRIVATE_KEY_LENGTH] = bytes
    .try_into()
    .map_err(|_| crate::Error::InvalidKeyLength(bytes.len(), Ed25519::PRIVATE_KEY_LENGTH))?;
  Ok(ed25519::SecretKey::from_bytes(private_key_bytes))
}

/// Reconstructs an Ed25519 public key from a byte array.
pub(crate) fn ed25519_public_try_from_bytes(bytes: &[u8]) -> Result<ed25519::PublicKey> {
  let public_key_bytes: [u8; Ed25519::PUBLIC_KEY_LENGTH] = bytes
    .try_into()
    .map_err(|_| crate::Error::InvalidKeyLength(bytes.len(), Ed25519::PUBLIC_KEY_LENGTH))?;
  ed25519::PublicKey::try_from_bytes(public_key_bytes).map_err(Into::into)
}

#[cfg(test)]
mod tests {
  use super::*;

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
