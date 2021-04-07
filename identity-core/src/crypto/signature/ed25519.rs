// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;
use core::marker::PhantomData;
use crypto::signatures::ed25519;
use crypto::signatures::ed25519::COMPRESSED_PUBLIC_KEY_LENGTH as PUBLIC_KEY_LENGTH;
use crypto::signatures::ed25519::SECRET_KEY_LENGTH;
use crypto::signatures::ed25519::SIGNATURE_LENGTH;

use crate::crypto::Sign;
use crate::crypto::Verify;
use crate::error::Error;
use crate::error::Result;

/// An implementation of `Ed25519` signatures.
#[derive(Clone, Copy, Debug)]
pub struct Ed25519<T: ?Sized = [u8]>(PhantomData<T>);

impl<T> Sign for Ed25519<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  type Secret = T;
  type Output = [u8; SIGNATURE_LENGTH];

  fn sign(message: &[u8], key: &Self::Secret) -> Result<Self::Output> {
    parse_secret(key.as_ref()).map(|key| key.sign(message).to_bytes())
  }
}

impl<T> Verify for Ed25519<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  type Public = T;

  fn verify(message: &[u8], signature: &[u8], key: &Self::Public) -> Result<()> {
    let key: ed25519::PublicKey = parse_public(key.as_ref())?;
    let sig: ed25519::Signature = parse_signature(signature)?;

    if key.verify(&sig, message) {
      Ok(())
    } else {
      Err(Error::InvalidProofValue("ed25519"))
    }
  }
}

fn parse_public(slice: &[u8]) -> Result<ed25519::PublicKey> {
  let bytes: [u8; PUBLIC_KEY_LENGTH] = slice
    .get(..PUBLIC_KEY_LENGTH)
    .and_then(|bytes| bytes.try_into().ok())
    .ok_or_else(|| Error::InvalidKeyLength(slice.len(), PUBLIC_KEY_LENGTH))?;

  ed25519::PublicKey::from_compressed_bytes(bytes).map_err(Into::into)
}

fn parse_secret(slice: &[u8]) -> Result<ed25519::SecretKey> {
  let bytes: [u8; SECRET_KEY_LENGTH] = slice
    .get(..SECRET_KEY_LENGTH)
    .and_then(|bytes| bytes.try_into().ok())
    .ok_or_else(|| Error::InvalidKeyLength(slice.len(), SECRET_KEY_LENGTH))?;

  ed25519::SecretKey::from_le_bytes(bytes).map_err(Into::into)
}

fn parse_signature(slice: &[u8]) -> Result<ed25519::Signature> {
  let bytes: [u8; SIGNATURE_LENGTH] = slice
    .get(..SIGNATURE_LENGTH)
    .and_then(|bytes| bytes.try_into().ok())
    .ok_or_else(|| Error::InvalidSigLength(slice.len(), SIGNATURE_LENGTH))?;

  Ok(ed25519::Signature::from_bytes(bytes))
}

#[cfg(test)]
mod tests {
  use crate::crypto::Ed25519;
  use crate::crypto::Sign;
  use crate::crypto::Verify;
  use crate::utils::decode_b58;

  const SIGNATURE_HELLO: &[u8] = &[
    12, 203, 235, 144, 80, 6, 163, 39, 181, 17, 44, 123, 250, 162, 165, 145, 135, 132, 32, 152, 24, 168, 55, 80, 84,
    139, 153, 101, 102, 27, 157, 29, 70, 124, 64, 120, 250, 172, 186, 163, 108, 27, 208, 248, 134, 115, 3, 154, 222,
    165, 31, 93, 33, 108, 212, 92, 191, 14, 21, 40, 251, 103, 241, 10, 104, 101, 108, 108, 111,
  ];

  const PUBLIC_B58: &str = "6b23ioXQSAayuw13PGFMCAKqjgqoLTpeXWCy5WRfw28c";
  const SECRET_B58: &str = "3qsrFcQqVuPpuGrRkU4wkQRvw1tc1C5EmEDPioS1GzQ2pLoThy5TYS2BsrwuzHYDnVqcYhMSpDhTXGst6H5ttFkG";

  #[test]
  fn test_ed25519_can_sign_and_verify() {
    let public: Vec<u8> = decode_b58(PUBLIC_B58).unwrap();
    let secret: Vec<u8> = decode_b58(SECRET_B58).unwrap();

    let signature: _ = Ed25519::sign(b"hello", &secret).unwrap();
    let combined: _ = [&signature[..], b"hello"].concat();

    assert_eq!(&combined, SIGNATURE_HELLO);

    let verified: _ = Ed25519::verify(b"hello", &signature, &public);
    assert!(verified.is_ok());
  }
}
