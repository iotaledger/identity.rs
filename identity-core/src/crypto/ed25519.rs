// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;
use ed25519_zebra::Signature;
use ed25519_zebra::SigningKey;
use ed25519_zebra::VerificationKey;

use crate::error::Error;
use crate::error::Result;

const SIGNATURE_LEN: usize = 64;
const PUBLIC_KEY_LEN: usize = 32;
const SECRET_KEY_LEN: usize = 32;

pub(crate) fn ed25519_sign(message: &[u8], secret: &[u8]) -> Result<[u8; SIGNATURE_LEN]> {
  let key: SigningKey = parse_secret(secret).ok_or(Error::InvalidKeyFormat)?;
  let sig: [u8; SIGNATURE_LEN] = key.sign(message).into();

  Ok(sig)
}

pub(crate) fn ed25519_verify(message: &[u8], signature: &[u8], public: &[u8]) -> Result<()> {
  let key: VerificationKey = parse_public(public).ok_or(Error::InvalidKeyFormat)?;
  let sig: Signature = parse_signature(signature).ok_or(Error::InvalidProofFormat)?;

  key.verify(&sig, message).map_err(|_| Error::InvalidProofValue)?;

  Ok(())
}

fn parse_public(slice: &[u8]) -> Option<VerificationKey> {
  slice.get(..PUBLIC_KEY_LEN).and_then(|bytes| bytes.try_into().ok())
}

fn parse_secret(slice: &[u8]) -> Option<SigningKey> {
  slice.get(..SECRET_KEY_LEN).and_then(|bytes| bytes.try_into().ok())
}

fn parse_signature(slice: &[u8]) -> Option<Signature> {
  slice.get(..SIGNATURE_LEN).and_then(|bytes| bytes.try_into().ok())
}

#[cfg(test)]
mod tests {
  use crate::crypto::ed25519_sign;
  use crate::crypto::ed25519_verify;
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

    let signature: _ = ed25519_sign(b"hello", &secret).unwrap();
    let combined: _ = [&signature[..], b"hello"].concat();

    assert_eq!(&combined, SIGNATURE_HELLO);

    let verified: _ = ed25519_verify(b"hello", &signature, &public);
    assert!(verified.is_ok());
  }
}
