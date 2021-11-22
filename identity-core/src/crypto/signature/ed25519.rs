// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;
use core::marker::PhantomData;
use crypto::signatures::ed25519;
use crypto::signatures::ed25519::PUBLIC_KEY_LENGTH;
use crypto::signatures::ed25519::SECRET_KEY_LENGTH;
use crypto::signatures::ed25519::SIGNATURE_LENGTH;

use crate::crypto::Sign;
use crate::crypto::Verify;
use crate::crypto::key::KeyError;

use self::errors::SignatureParsingError;
use super::errors::InvalidProofValue;
use super::errors::VerificationError;
use super::errors::SigningError;

/// An implementation of `Ed25519` signatures.
#[derive(Clone, Copy, Debug)]
pub struct Ed25519<T: ?Sized = [u8]>(PhantomData<T>);

impl<T> Sign for Ed25519<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  type Private = T;
  type Error = SigningError;  
  type Output = [u8; SIGNATURE_LENGTH];

  fn sign(message: &[u8], key: &Self::Private) -> std::result::Result<Self::Output, Self::Error> {
    parse_secret(key.as_ref()).map(|key| key.sign(message).to_bytes()).map_err(Into::into)
  }
}

impl<T> Verify for Ed25519<T>
where
  T: AsRef<[u8]> + ?Sized,
{
  type Public = T;

  type Error = VerificationError; 

  fn verify(message: &[u8], signature: &[u8], key: &Self::Public) -> std::result::Result<(), Self::Error> {
    let key: ed25519::PublicKey = parse_public(key.as_ref()).map_err(Self::Error::from)?;
    let sig: ed25519::Signature = parse_signature(signature).map_err(Self::Error::from)?; 

    if key.verify(&sig, message) {
      Ok(())
    } else {
      Err(InvalidProofValue("ed25519").into())
    }
  }
}

fn parse_public(slice: &[u8]) -> std::result::Result<ed25519::PublicKey, KeyError> {
  let bytes: [u8; PUBLIC_KEY_LENGTH] = slice.try_into().map_err(|_| KeyError("could not create a public key from the supplied bytes: incorrect length"))?;
  ed25519::PublicKey::try_from_bytes(bytes).map_err(|_|KeyError("could not parse public key from the supplied bytes"))
}

fn parse_secret(slice: &[u8]) -> std::result::Result<ed25519::SecretKey, KeyError> {
  let bytes: [u8; SECRET_KEY_LENGTH] = slice.try_into().map_err(|_|KeyError("could not create a secret key from the supplied bytes: incorrect length"))?;

  Ok(ed25519::SecretKey::from_bytes(bytes))
}

fn parse_signature(slice: &[u8]) -> std::result::Result<ed25519::Signature, SignatureParsingError> {
  let bytes: [u8; SIGNATURE_LENGTH] = slice.try_into().map_err(|_| SignatureParsingError("could not parse a signature from the supplied bytes"))?;
  Ok(ed25519::Signature::from_bytes(bytes))
}

mod errors {
use thiserror::Error as DeriveError;

use crate::crypto::signature::errors::{VerificationError, VerificationProcessingError};
  #[derive(Debug,DeriveError)]
  /// Caused by a failure to parse a signature
  #[error("{0}")]
  pub(super) struct SignatureParsingError(pub &'static str); 
  
  impl From<SignatureParsingError> for VerificationError {
    fn from(err: SignatureParsingError) -> Self {
       Self::ProcessingFailed(VerificationProcessingError::InvalidInputFormat(err.0))
    }
  }
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
    let private: Vec<u8> = decode_b58(SECRET_B58).unwrap();

    let signature: _ = Ed25519::sign(b"hello", &private).unwrap();
    let combined: _ = [&signature[..], b"hello"].concat();

    assert_eq!(&combined, SIGNATURE_HELLO);

    let verified: _ = Ed25519::verify(b"hello", &signature, &public);
    assert!(verified.is_ok());
  }
}
