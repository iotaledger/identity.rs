// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

use crate::jwk::Jwk;
use crate::jwt::JwtHeaderSet;

use super::JwsHeader;
use crate::jwk::EdCurve;
use crate::jwk::JwkParamsOkp;
use crate::jws::JwsAlgorithm;

pub type HeaderSet<'a> = JwtHeaderSet<'a, JwsHeader>;

/// Verification data for a [`JwsSignatureVerifier`].
pub struct VerificationInput<'a> {
  pub(super) jose_header: &'a HeaderSet<'a>,
  pub(super) signing_input: Vec<u8>,
  pub(super) signature: &'a [u8],
}

impl<'a> VerificationInput<'a> {
  /// The JOSE header.
  pub fn jose_header(&self) -> &HeaderSet<'a> {
    &self.jose_header
  }

  /// The signing input.
  pub fn signing_input(&self) -> &[u8] {
    self.signing_input.as_ref()
  }

  /// The decoded signature.
  pub fn signature(&self) -> &'a [u8] {
    self.signature
  }
}

/// Error type for a failed jws signature verification. See [`JwsSignatureVerifier`].
#[derive(Debug, thiserror::Error)]
#[error("jws signature verification failed: {kind}")]
pub struct JwsVerifierError {
  kind: JwsVerifierErrorKind,
  #[source]
  source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl JwsVerifierError {
  /// Constructs a new [`JwsVerifierError`].
  pub fn new(cause: JwsVerifierErrorKind) -> Self {
    Self {
      kind: cause,
      source: None,
    }
  }

  /// Returns the cause of the [`JwsVerifierError`].
  pub fn kind(&self) -> &JwsVerifierErrorKind {
    &self.kind
  }
  /// Updates the `source` of the [`JwsVerifierError`].
  pub fn with_source(self, source: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
    self._with_source(source.into())
  }

  fn _with_source(mut self, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
    self.source = Some(source);
    self
  }
}

impl From<JwsVerifierErrorKind> for JwsVerifierError {
  fn from(value: JwsVerifierErrorKind) -> Self {
    Self::new(value)
  }
}

/// The cause of a failed jws signature verification.
#[derive(Debug)]
#[non_exhaustive]
pub enum JwsVerifierErrorKind {
  /// Indicates that the [`JwsSignatureVerifier`] implementation is not compatible with the `alg` extracted from the
  /// JOSE header.
  UnsupportedAlg,
  /// Indicates that the [`JwsSignatureVerifier`] implementation does not support the `kty` of the provided [`Jwk`].
  UnsupportedKeyType,
  /// Indicates that the [`JwsSignatureVerifier`] implementation does not support the public key parameters extracted
  /// from the provided [`Jwk`].
  UnsupportedKeyParams,
  /// Indicates that the [`JwsSignatureVerifier`] implementation failed to decode the public key extracted from the
  /// provided [`Jwk`].
  KeyDecodingFailure,
  /// Indicates that the [`JwsSignatureVerifier`] implementation considers the signature to be invalid.
  InvalidSignature,
  /// Indicates that something went wrong when calling
  /// [`JwsSignatureVerifier::verify`](JwsSignatureVerifier::verify()), but it is unclear whether the reason matches
  /// any of the other variants.
  Unspecified,
}

impl JwsVerifierErrorKind {
  const fn as_str(&self) -> &str {
    match self {
      Self::UnsupportedAlg => "unsupported alg",
      Self::UnsupportedKeyType => "unsupported key type",
      Self::UnsupportedKeyParams => "unsupported key parameters",
      Self::KeyDecodingFailure => "key decoding failure",
      Self::InvalidSignature => "invalid signature",
      Self::Unspecified => "unspecified failure",
    }
  }
}

impl Display for JwsVerifierErrorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}

/// Trait for verifying a JWS signature with a given public key represented as a `JWK`.
///
/// When the `default-jws-signature-verifier` feature is enabled one can construct a default implementor
/// provided by the IOTA Identity library. See
/// [`DefaultJwsSignatureVerifier::verify`](DefaultSignatureVerifier::verify()).
///
/// For custom implementations the most ergonomic option is in many cases converting a suitable closure to a
/// [`JwsSignatureVerifierFn`] using the [`From`] trait.
pub trait JwsSignatureVerifier {
  fn verify<'a>(&self, input: &VerificationInput<'a>, public_key: &Jwk) -> Result<(), JwsVerifierError>;
}

/// Simple wrapper around a closure capable of verifying a JWS signature. This wrapper implements
/// [`JwsSignatureVerifier`].
///
/// Note: One can convert a closure to this wrapper using the [`From`]. Currently the closure's input arguments must
/// be explicitly typed otherwise a compiler error occurs.  
pub struct JwsSignatureVerifierFn<F>(F);
impl<F> From<F> for JwsSignatureVerifierFn<F>
where
  for<'a> F: Fn(&VerificationInput<'a>, &Jwk) -> Result<(), JwsVerifierError>,
{
  fn from(value: F) -> Self {
    Self(value)
  }
}

impl<F> JwsSignatureVerifier for JwsSignatureVerifierFn<F>
where
  for<'a> F: Fn(&VerificationInput<'a>, &Jwk) -> Result<(), JwsVerifierError>,
{
  fn verify<'a>(&self, input: &VerificationInput<'a>, public_key: &Jwk) -> Result<(), JwsVerifierError> {
    self.0(input, public_key)
  }
}

/// The default implementor of [`JwsSignatureVerifier`] provided by the IOTA Identity library.
///
/// NOTE: This type can only be constructed when the `default-jws-signature-verifier` feature is enabled.
pub struct DefaultJwsSignatureVerifier {
  _internal: (),
}

impl DefaultJwsSignatureVerifier {
  /// Verify a JWS signature secured with the [`JwsAlgorithm::EdDSA`] algorithm.
  /// Only [`EdCurve::Ed25519`] variant is supported for now. This static method is only available when the
  /// `eddsa` feature is enabled.
  #[cfg(any(feature = "eddsa", doc))]
  pub fn verify_eddsa_jws_prechecked_alg(
    input: &VerificationInput<'_>,
    public_key: &Jwk,
  ) -> Result<(), JwsVerifierError> {
    // Obtain an Ed25519 public key
    let params: &JwkParamsOkp = public_key
      .try_okp_params()
      .map_err(|_| JwsVerifierErrorKind::UnsupportedKeyType)?;

    if params
      .try_ed_curve()
      .ok()
      .filter(|curve_param| *curve_param == EdCurve::Ed25519)
      .is_none()
    {
      return Err(JwsVerifierErrorKind::UnsupportedKeyParams.into());
    }

    let pk: [u8; crypto::signatures::ed25519::PUBLIC_KEY_LENGTH] = crate::jwu::decode_b64(params.x.as_str())
      .map_err(|_| JwsVerifierErrorKind::KeyDecodingFailure)
      .and_then(|value| TryInto::try_into(value).map_err(|_| JwsVerifierErrorKind::KeyDecodingFailure))?;

    let public_key_ed25519 =
      crypto::signatures::ed25519::PublicKey::try_from(pk).map_err(|_| JwsVerifierErrorKind::KeyDecodingFailure)?;

    let signature_arr = <[u8; crypto::signatures::ed25519::SIGNATURE_LENGTH]>::try_from(input.signature())
      .map_err(|_| JwsVerifierErrorKind::InvalidSignature)?;

    let signature = crypto::signatures::ed25519::Signature::from_bytes(signature_arr);

    if crypto::signatures::ed25519::PublicKey::verify(&public_key_ed25519, &signature, input.signing_input()) {
      Ok(())
    } else {
      Err(JwsVerifierErrorKind::InvalidSignature.into())
    }
  }
}

#[cfg(any(feature = "default-jws-signature-verifier", doc))]
impl Default for DefaultJwsSignatureVerifier {
  /// Constructs a [`DefaultJwsVerifier`]. This is the only way to obtain a `DefaultJwsVerifier` and is only available
  /// when the `default-jws-signature-verifier` feature is set.
  fn default() -> Self {
    Self { _internal: () }
  }
}

impl JwsSignatureVerifier for DefaultJwsSignatureVerifier {
  /// Default implementer of [`JwsSignatureVerifier`]. For now `DefaultJwsVerifier::verify` can only handle the `alg =
  /// EdDSA` with `crv = Ed25519`, but the implementation may support more algorithms in the future.
  #[cfg(feature = "default-jws-signature-verifier")]
  fn verify(&self, input: &VerificationInput<'_>, public_key: &Jwk) -> std::result::Result<(), JwsVerifierError> {
    let alg = input.jose_header().alg().ok_or(JwsVerifierErrorKind::UnsupportedAlg)?;
    match alg {
      // EdDSA is the only supported algorithm for now, we can consider supporting more by default in the future.
      JwsAlgorithm::EdDSA => DefaultJwsSignatureVerifier::verify_eddsa_jws_prechecked_alg(input, public_key),
      _ => Err(JwsVerifierErrorKind::UnsupportedAlg.into()),
    }
  }

  // This method can never be called because it is impossible to construct the `DefaultJwsVerifier` without enabling
  // the `default-jws-signature-verifier` feature. It is still necessary to implement this method in order to satisfy
  // the trait bounds on the default parameter for the `Decoder`.
  #[cfg(not(feature = "default-jws-signature-verifier"))]
  fn verify(&self, input: &VerificationInput<'_>, public_key: &Jwk) -> std::result::Result<(), JwsVerifierError> {
    panic!("it should not be possible to construct a DefaultJwsVerifier without the 'default-jws-signature-verifier' feature. We encourage you to report this bug at: https://github.com/iotaledger/identity.rs/issues");
  }
}
