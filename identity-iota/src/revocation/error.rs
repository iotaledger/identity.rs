// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) type Result<T, E = RevocationError> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum RevocationError {
  #[error("unable to decode base64 `String`: {0}")]
  Base64DecodingError(String, #[source] identity_core::error::Error),
  #[error("unable to compress data")]
  CompressionError(#[source] std::io::Error),
  #[error("unable to compress data")]
  DecompressionError(#[source] std::io::Error),
  #[error("revocation list could not be deserialized")]
  DeserializationError(#[source] std::io::Error),
  #[error("revocation list could not be serialized")]
  SerializationError(#[source] std::io::Error),
  #[error("method {0} is not supported for revocation")]
  UnsupportedRevocationMethod(String),
}
