// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;
use core::marker::PhantomData;

use libsecp256k1;

use crate::crypto::Sign;
use crate::crypto::Verify;
use crate::error::Error;
use crate::error::Result;

/// An implementation of `Ed25519` signatures.
#[derive(Clone, Copy, Debug)]
pub struct Secp256k1<T: ?Sized = [u8]>(PhantomData<T>);

impl Secp256k1 {
  /// Length in bytes of an Secp256k1 private key.
  pub const PRIVATE_KEY_LENGTH: usize = libsecp256k1::util::SECRET_KEY_SIZE;
  /// Length in bytes of an Secp256k1 compressed public key.
  pub const PUBLIC_KEY_LENGTH: usize = libsecp256k1::util::COMPRESSED_PUBLIC_KEY_SIZE;
  /// Length in bytes of an Secp256k1 signature.
  pub const SIGNATURE_LENGTH: usize = libsecp256k1::util::SIGNATURE_SIZE;
}

impl<T> Sign for Secp256k1<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  type Private = T;
  type Output = [u8; Secp256k1::SIGNATURE_LENGTH];
  /// Computes an ECDSA signature using an Secp256k1 private key.
  ///
  /// Uses the SHA-256 digest of the message, as per [RFC8812 Section 3.2](https://datatracker.ietf.org/doc/html/rfc8812#section-3.2).
  fn sign(message: &[u8], key: &Self::Private) -> Result<Self::Output> {
    let secret_key: libsecp256k1::SecretKey = secp256k1_private_try_from_bytes(key.as_ref())?;
    let msg: libsecp256k1::Message = hash_message(message);
    let signature: libsecp256k1::Signature = libsecp256k1::sign(&msg, &secret_key).0;
    Ok(signature.serialize())
  }
}

/// Computes the SHA-256 digest of a message, as per [RFC8812 Section 3.2](https://datatracker.ietf.org/doc/html/rfc8812#section-3.2).
fn hash_message(message: &[u8]) -> libsecp256k1::Message {
  let mut digest: [u8; 32] = [0; 32];
  crypto::hashes::sha::SHA256(message, &mut digest);
  libsecp256k1::Message::parse(&digest)
}

impl<T> Verify for Secp256k1<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  type Public = T;

  /// Verifies an ECDSA signature against an Secp256k1 compressed public key.
  ///
  /// Uses the SHA-256 digest of the message, as per [RFC8812 Section 3.2](https://datatracker.ietf.org/doc/html/rfc8812#section-3.2).
  fn verify(message: &[u8], signature: &[u8], key: &Self::Public) -> Result<()> {
    let key: libsecp256k1::PublicKey = secp256k1_public_try_from_bytes(key.as_ref())?;
    let sig: libsecp256k1::Signature = parse_signature(signature)?;
    let msg: libsecp256k1::Message = hash_message(message);

    if libsecp256k1::verify(&msg, &sig, &key) {
      Ok(())
    } else {
      Err(Error::InvalidProofValue("Secp256k1"))
    }
  }
}

fn parse_signature(slice: &[u8]) -> Result<libsecp256k1::Signature> {
  let bytes: [u8; Secp256k1::SIGNATURE_LENGTH] = slice
    .try_into()
    .map_err(|_| Error::InvalidSigLength(slice.len(), Secp256k1::SIGNATURE_LENGTH))?;

  libsecp256k1::Signature::parse_standard(&bytes)
    .map_err(|_| Error::InvalidProofValue("Secp256k1 invalid signature bytes"))
}

/// Reconstructs an Secp256k1 private key from a byte array.
pub(crate) fn secp256k1_private_try_from_bytes(bytes: &[u8]) -> Result<libsecp256k1::SecretKey> {
  let private_key_bytes: [u8; Secp256k1::PRIVATE_KEY_LENGTH] = bytes
    .try_into()
    .map_err(|_| Error::InvalidKeyLength(bytes.len(), Secp256k1::PRIVATE_KEY_LENGTH))?;
  libsecp256k1::SecretKey::parse(&private_key_bytes)
    .map_err(|err| Error::InvalidKeyFormat(format!("Secp256k1 - {err}")))
}

/// Reconstructs a compressed Secp256k1 public key from a byte array.
pub(crate) fn secp256k1_public_try_from_bytes(bytes: &[u8]) -> Result<libsecp256k1::PublicKey> {
  let public_key_bytes: [u8; Secp256k1::PUBLIC_KEY_LENGTH] = bytes
    .try_into()
    .map_err(|_| Error::InvalidKeyLength(bytes.len(), Secp256k1::PUBLIC_KEY_LENGTH))?;
  libsecp256k1::PublicKey::parse_compressed(&public_key_bytes)
    .map_err(|err| Error::InvalidKeyFormat(format!("Secp256k1 - {err}")))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::Base::Base16Lower;
  use crate::utils::BaseEncoding;
  use std::ops::Deref;

  const SECRET_KEY_HEX: &str = "ebb2c082fd7727890a28ac82f6bdf97bad8de9f5d7c9028692de1a255cad3e0f";
  const PUBLIC_KEY_HEX: &str = "03779dd197a5df977ed2cf6cb31d82d43328b790dc6b3b7d4437a427bd5847dfcd";
  const MESSAGE: &str = "test123";
  const SIGNATURE_HEX: &str = "3f903a40d2034f343bc722d69c29b0340c2e7a8ef3d1ee79f856cacfc042bb5b491aed1ebd347fd0b50656b599bfe60cbe8635cb4ca4cb6e621a01503b498e5c";

  #[test]
  fn test_secp256k1_keys() {
    let private_key = BaseEncoding::decode(SECRET_KEY_HEX, Base16Lower).unwrap();
    let public_key = BaseEncoding::decode(PUBLIC_KEY_HEX, Base16Lower).unwrap();
    let sk: libsecp256k1::SecretKey = secp256k1_private_try_from_bytes(&private_key).unwrap();
    let pk: libsecp256k1::PublicKey = secp256k1_public_try_from_bytes(&public_key).unwrap();
    assert_eq!(sk.serialize(), private_key.deref());
    assert_eq!(pk.serialize_compressed(), public_key.deref());

    let pk2: libsecp256k1::PublicKey = libsecp256k1::PublicKey::from_secret_key(&sk);
    assert_eq!(pk.serialize_compressed(), pk2.serialize_compressed());
  }

  #[test]
  fn test_secp256k1_sign_verify() {
    let private_key = BaseEncoding::decode(SECRET_KEY_HEX, Base16Lower).unwrap();
    let public_key = BaseEncoding::decode(PUBLIC_KEY_HEX, Base16Lower).unwrap();
    let signature = Secp256k1::sign(MESSAGE.as_bytes(), &private_key).unwrap();
    assert_eq!(&BaseEncoding::encode(&signature, Base16Lower), SIGNATURE_HEX);
    let verified: _ = Secp256k1::verify(MESSAGE.as_bytes(), &signature, &public_key);
    assert!(verified.is_ok());
  }
}
