// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  #[error("unable to decode base-64 `String`: {0}")]
  Base64DecodingError(String, #[source] identity_core::error::Error),
  #[error("revocation list could not be deserialized")]
  DeserializationError(#[source] std::io::Error),
  #[error("revocation list could not be serialized")]
  SerializationError(#[source] std::io::Error),
  #[error("method {0} is not supported for revocation")]
  UnsupportedRevocationMethod(String),
  #[error("credential at index {0} has been revoked")]
  RevokedCredential(u32),
}
