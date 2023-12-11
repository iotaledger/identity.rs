// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::validator::JwtValidationError;

///
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
pub enum KeyBindingJwtError {
  ///
  #[error("KB-JWT is invalid")]
  JwtValidationError(#[from] JwtValidationError),

  ///
  #[error("Deserialization error")]
  DeserializationError(String),

  ///
  #[error("SdJwt Error {0}")]
  SdJwtError(#[from] sd_jwt::Error),

  ///
  #[error("the `_sd_hash` value of the KB-JWT does not match the derived value from the provided SD-JWT")]
  InvalidDigest,

  ///
  #[error("provided nonce does not match the KB-JWT nonce claim")]
  InvalidNonce,

  ///
  #[error("provided audiance value does not match the KB-JWT `aud` claim")]
  AudianceMismatch,

  ///
  #[error("KB-JWT `iat` value is in the fututre or earlier than the provided `latest_issuance_date`")]
  IssuanceDate,

  ///
  #[error("the provided SD-JWT does not include a KB-JWT")]
  MissingKeyBindingJwt,

  ///
  #[error("header `typ` value is missing or not equal to `kb+jwt`")]
  InvalidHeaderTypValue,
}
