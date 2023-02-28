// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur in the library.

/// Alias for a `Result` with the error type [Error].
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// All possible errors that can occur in the library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("invalid json")]
  InvalidJson(#[source] serde_json::Error),
  #[error("invalid base64")]
  InvalidBase64(#[source] base64::DecodeError),
  #[error("invalid utf-8")]
  InvalidUtf8(#[source] core::str::Utf8Error),
  #[error("invalid claim `{0}`")]
  InvalidClaim(&'static str),
  #[error("missing claim `{0}`")]
  MissingClaim(&'static str),
  #[error("invalid param: {0}")]
  InvalidParam(&'static str),
  #[error("missing param `{0}`")]
  MissingParam(&'static str),
  #[error("{0}")]
  InvalidContent(&'static str),
  #[error("invalid key format for type `{0}`")]
  KeyError(&'static str),
  #[error("failed to obtain a Jwk: Jwk was not provided or extracted otherwise")]
  JwkNotProvided,
  #[error("mismatch between header and jwk alg values")]
  AlgorithmMismatch,
  #[error("could not extract alg from a jwk required to have this field")]
  JwkWithoutAlg,
  #[error("unsupported jws algorithm")]
  UnsupportedAlgorithm,
  #[error("signature creation error")]
  SignatureCreationError(#[source] Box<dyn std::error::Error + Send + Sync>),
  #[error("signature verification error")]
  SignatureVerificationError(#[source] crate::jws::SignatureVerificationError)
}
