// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;

use did_url::Error;

#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
/// Error type caused by invalid DID handling.
pub enum DIDError {
  #[error("Invalid Authority")]
  InvalidAuthority,
  #[error("Invalid Fragment")]
  InvalidFragment,
  #[error("Invalid Method Id")]
  InvalidMethodId,
  #[error("Invalid Method Name")]
  InvalidMethodName,
  #[error("Invalid Path")]
  InvalidPath,
  #[error("Invalid Query")]
  InvalidQuery,
  #[error("Invalid Scheme")]
  InvalidScheme,

  #[error("{0}")]
  Other(&'static str),
}

impl From<did_url::Error> for DIDError {
  fn from(error: Error) -> Self {
    match error {
      Error::InvalidAuthority => Self::InvalidAuthority,
      Error::InvalidFragment => Self::InvalidFragment,
      Error::InvalidMethodId => Self::InvalidMethodId,
      Error::InvalidMethodName => Self::InvalidMethodName,
      Error::InvalidPath => Self::InvalidPath,
      Error::InvalidQuery => Self::InvalidQuery,
      Error::InvalidScheme => Self::InvalidScheme,
      error => Self::Other(error.as_str()),
    }
  }
}
