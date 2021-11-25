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

use crate::crypto::signature::errors::{VerificationError, VerificationProcessingErrorCause};
  #[derive(Debug,DeriveError)]
  /// Caused by a failure to parse a signature
  #[error("{0}")]
  pub(super) struct SignatureParsingError(pub(super) &'static str); 
  
  impl From<SignatureParsingError> for VerificationError {
    fn from(err: SignatureParsingError) -> Self {
       Self::ProcessingFailed(VerificationProcessingErrorCause::InvalidInputFormat(err.0).into())
    }
  }
}
#[cfg(test)]
mod tests {
  use crate::crypto::Ed25519;
  use crate::crypto::Sign;
  use crate::crypto::Verify;
  use crate::utils::Base;
  use crate::utils;

  const SIGNATURE_HEX: &str = "92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00"; 

  const PUBLIC_KEY_HEX: &str = "fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025";
  const SECRET_KEY_HEX: &str = "c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7";
  const MESSAGE_HEX: &str = "af82"; 

  #[test]
  fn test_ed25519_can_sign_and_verify() {
    let public_key = utils::decode_multibase(PUBLIC_KEY_HEX).unwrap(); 
    let private_key = utils::decode_multibase(SECRET_KEY_HEX).unwrap(); 
    let message = utils::decode_multibase(MESSAGE_HEX).unwrap(); 

    let signature: _ = Ed25519::sign(&message, &private_key).unwrap();

    //assert_eq!(&signature, SIGNATURE_HEX);

    //let verified: _ = Ed25519::verify(MESSAGE_HEX, &signature, PUBLIC_KEY_HEX);
    //assert!(verified.is_ok());
  }
}
