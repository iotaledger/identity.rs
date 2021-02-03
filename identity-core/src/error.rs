// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur when Self-sovereign Identity goes wrong.

/// Alias for a `Result` with the error type [`Error`].
pub type Result<T, E = Error> = ::core::result::Result<T, E>;

/// This type represents all possible errors that can occur in the library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Caused by a failure to encode Rust types as JSON.
  #[error("Failed to encode JSON: {0}")]
  EncodeJSON(serde_json::Error),
  /// Caused by a failure to decode Rust types from JSON.
  #[error("Failed to decode JSON: {0}")]
  DecodeJSON(serde_json::Error),
  /// Caused by a failure to decode base16-encoded data.
  #[error("Failed to decode base16 data: {0}")]
  DecodeBase16(#[from] hex::FromHexError),
  /// Caused by a failure to decode base58-encoded data.
  #[error("Failed to decode base58 data: {0}")]
  DecodeBase58(#[from] bs58::decode::Error),
  /// Caused by a failure to decode base64-encoded data.
  #[error("Failed to decode base64 data: {0}")]
  DecodeBase64(#[from] base64::DecodeError),
  /// Cause by a failure to encode a Roaring Bitmap.
  #[error("Failed to encode roaring bitmap: {0}")]
  EncodeBitmap(std::io::Error),
  /// Cause by a failure to decode a Roaring Bitmap.
  #[error("Failed to decode roaring bitmap: {0}")]
  DecodeBitmap(std::io::Error),
  /// Caused by attempting to perform an invalid `Diff` operation.
  #[error("Invalid Document Diff: {0}")]
  InvalidDiff(#[from] identity_diff::Error),
  /// Caused by attempting to parse an invalid `Url`.
  #[error("Invalid Url: {0}")]
  InvalidUrl(#[from] url::ParseError),
  /// Caused by attempting to parse an invalid `Timestamp`.
  #[error("Invalid Timestamp: {0}")]
  InvalidTimestamp(#[from] chrono::ParseError),
  /// Caused by attempting to parse an invalid DID proof.
  #[error("Invalid Proof Format")]
  InvalidProofFormat,
  /// Caused by attempting to parse an invalid cryptographic key.
  #[error("Invalid Key Format")]
  InvalidKeyFormat,
  /// Caused by a failed attempt at retrieving a digital signature.
  #[error("Signature Not Found")]
  MissingSignature,
}
