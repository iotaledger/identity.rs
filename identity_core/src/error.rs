// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur when Self-sovereign Identity goes wrong.

use crate::utils::Base;

/// Alias for a `Result` with the error type [`Error`].
pub type Result<T, E = Error> = ::core::result::Result<T, E>;

/// This type represents all possible errors that can occur in the library.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
pub enum Error {
  /// Caused when a cryptographic operation fails.
  #[error("crypto error")]
  Crypto(#[source] crypto::Error),
  /// Caused by a failure to encode Rust types as JSON.
  #[error("failed to encode JSON")]
  EncodeJSON(#[source] serde_json::Error),
  /// Caused by a failure to decode Rust types from JSON.
  #[error("failed to decode JSON")]
  DecodeJSON(#[source] serde_json::Error),
  /// Caused by a failure to decode base-encoded data.
  #[error("Failed to decode {0:?} data")]
  DecodeBase(Base, #[source] multibase::Error),
  /// Caused by a failure to decode multibase-encoded data.
  #[error("failed to decode multibase data")]
  DecodeMultibase(#[from] multibase::Error),
  /// Caused by attempting to parse an invalid `Url`.
  #[error("invalid url")]
  InvalidUrl(#[from] url::ParseError),
  /// Caused by attempting to parse an invalid `Timestamp`.
  #[error("invalid timestamp")]
  InvalidTimestamp(#[from] time::error::Error),
  /// Caused by attempting to create an empty `OneOrSet` instance or remove all its elements.
  #[error("OneOrSet cannot be empty")]
  OneOrSetEmpty,
  /// Caused by attempting to convert a collection with duplicate keys into an OrderedSet.
  #[error("duplicate key in OrderedSet")]
  OrderedSetDuplicate,
  /// Caused by attempting to parse an invalid `ProofPurpose`.
  #[error("invalid ProofPurpose")]
  InvalidProofPurpose,
  /// Raised by a validation attempt against an invalid DID proof.
  #[error("invalid proof value: {0}")]
  InvalidProofValue(&'static str),
  /// Caused by attempting to parse an invalid cryptographic key.
  #[error("invalid key format")]
  InvalidKeyFormat,
  /// Caused byt attempting to parse as invalid cryptographic key.
  #[error("invalid key length; Received {0}, expected {1}")]
  InvalidKeyLength(usize, usize),
  /// Caused byt attempting to parse as invalid digital signature.
  #[error("invalid signature length; Received {0}, expected {1}")]
  InvalidSigLength(usize, usize),
  /// Caused by a failed attempt at retrieving a digital signature.
  #[error("signature not found")]
  MissingSignature,
}

impl From<crypto::Error> for Error {
  fn from(other: crypto::Error) -> Self {
    Self::Crypto(other)
  }
}
