// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// The error type for the [`RemoteAccount`].
#[derive(Debug, thiserror::Error, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub(crate) enum RemoteAccountError {
  #[error("identity not found")]
  IdentityNotFound,
  #[error("{0}")]
  AccountError(String),
}

impl From<identity_account::Error> for RemoteAccountError {
  fn from(err: identity_account::Error) -> Self {
    Self::AccountError(err.to_string())
  }
}
