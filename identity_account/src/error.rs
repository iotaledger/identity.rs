// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur when working with Identity Accounts.

/// Alias for a `Result` with the error type [`Error`].
pub type Result<T, E = Error> = ::core::result::Result<T, E>;

/// This type represents all possible errors that can occur in the library.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  /// Caused by errors from the [identity_core] crate.
  #[error(transparent)]
  CoreError(#[from] identity_core::Error),
  /// Caused by errors from the [identity_did] crate.
  #[error(transparent)]
  DIDError(#[from] identity_did::Error),
  /// Caused by errors from the [identity_credential] crate.
  #[error(transparent)]
  CredentialError(#[from] identity_credential::Error),
  /// Caused by errors from the [identity_account_storage] crate.
  #[error(transparent)]
  AccountCoreError(#[from] identity_account_storage::Error),
  /// Caused by errors from the [identity_iota_client_legacy] crate.
  #[error(transparent)]
  IotaClientError(#[from] identity_iota_client_legacy::Error),
  /// Caused by errors from the [identity_iota_core_legacy] crate.
  #[error(transparent)]
  IotaCoreError(#[from] identity_iota_core_legacy::Error),
  /// Caused by attempting to find an identity that does not exist.
  #[error("Identity not found")]
  IdentityNotFound,
  /// Caused by attempting to perform an upate in an invalid context.
  #[error("Update Error: {0}")]
  UpdateError(#[from] crate::updates::UpdateError),
  /// Caused by verification methods without fragments.
  #[error("method missing fragment")]
  MethodMissingFragment,
  /// Caused by reaching an invalid state for the identity.
  #[error("invalid identity state: {0}")]
  InvalidIdentityState(String),
}

impl From<identity_did::did::DIDError> for Error {
  fn from(error: identity_did::did::DIDError) -> Self {
    identity_did::Error::from(error).into()
  }
}
