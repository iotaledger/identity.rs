// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;


#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
#[non_exhaustive]
/// Error type caused by invalid DID handling.
pub enum Error {
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

impl From<did_url::Error> for Error {
  fn from(error: did_url::Error) -> Self {
    match error {
      did_url::Error::InvalidAuthority => Self::InvalidAuthority,
      did_url::Error::InvalidFragment => Self::InvalidFragment,
      did_url::Error::InvalidMethodId => Self::InvalidMethodId,
      did_url::Error::InvalidMethodName => Self::InvalidMethodName,
      did_url::Error::InvalidPath => Self::InvalidPath,
      did_url::Error::InvalidQuery => Self::InvalidQuery,
      did_url::Error::InvalidScheme => Self::InvalidScheme,
      error => Self::Other(error.as_str()),
    }
  }
}
