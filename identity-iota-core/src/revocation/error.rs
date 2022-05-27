// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) type Result<T, E = RevocationMethodError> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum RevocationMethodError {
  #[error("unable to decode base64 `String`: {0}")]
  Base64DecodingError(String, #[source] identity_core::error::Error),
  #[error("unable to compress data")]
  CompressionError(#[source] std::io::Error),
  #[error("unable to decompress data")]
  DecompressionError(#[source] std::io::Error),
  #[error("revocation list could not be deserialized")]
  DeserializationError(#[source] std::io::Error),
  #[error("revocation list could not be serialized")]
  SerializationError(#[source] std::io::Error),
  #[error("revocation list could not be represented as a valid URL: {0}")]
  InvalidUrlRepresentation(String, #[source] identity_core::Error),
}
