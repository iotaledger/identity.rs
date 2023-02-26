// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jwk::Jwk;
use crate::jwt::JwtHeaderSet;

use super::JwsHeader;

pub type JwsUnprotectedHeader<'a> = &'a JwsHeader;

pub type HeaderSet<'a> = JwtHeaderSet<'a, JwsHeader>;

/// Input intended for an `alg` specific
/// JWS verifier.
pub struct VerificationInput<'a> {
  pub(super) jose_header: &'a HeaderSet<'a>,
  pub(super) signing_input: Vec<u8>,
  pub(super) signature: &'a [u8],
}

impl<'a> VerificationInput<'a> {
  pub fn jose_header(&self) -> &HeaderSet<'a> {
    &self.jose_header
  }

  pub fn signing_input(&self) -> &[u8] {
    self.signing_input.as_ref()
  }

  pub fn signature(&self) -> &'a [u8] {
    self.signature
  }
}

#[derive(Debug, thiserror::Error)]
pub enum JwsVerifierError {
  #[error("could not verify jws: unsupported alg")]
  UnsupportedAlg,
  #[error("could not verify jws: unsupported key type ")]
  UnsupportedKeyType,
  #[error("could not verify jws: unsupported key parameters")]
  UnsupportedKeyParams,
  #[error("could not verify jws: signature verification failed")]
  SignatureVerificationError(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
  #[error("could not verify jws: missing parameter {0}")]
  MissingJwkParameter(&'static str),
  #[error("could not verify jws")]
  Unspecified(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
}

pub trait JwsSignatureVerifier {
  fn verify(&self, input: &VerificationInput<'_>, public_key: &Jwk) -> Result<(), JwsVerifierError>;
}

/// Simple wrapper around a closure capable of verifying a JWS signature. This wrapper implements
/// [`JwsSignatureVerifier`].
pub struct JwsVerifierFn<F: Fn(&VerificationInput<'_>, &Jwk) -> Result<(), JwsVerifierError>>(F);
impl<F> From<F> for JwsVerifierFn<F>
where
  F: Fn(&VerificationInput<'_>, &Jwk) -> Result<(), JwsVerifierError>,
{
  fn from(value: F) -> Self {
    Self(value)
  }
}

impl<F> JwsSignatureVerifier for JwsVerifierFn<F>
where
  F: Fn(&VerificationInput<'_>, &Jwk) -> Result<(), JwsVerifierError>,
{
  fn verify(&self, input: &VerificationInput<'_>, public_key: &Jwk) -> Result<(), JwsVerifierError> {
    self.0(input, public_key)
  }
}
