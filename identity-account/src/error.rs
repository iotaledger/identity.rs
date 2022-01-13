// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur when working with Identity Accounts.

/// Alias for a `Result` with the error type [`Error`].
pub type Result<T, E = Error> = ::core::result::Result<T, E>;

/// This type represents all possible errors that can occur in the library.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum Error {
  /// Caused by errors from the [crypto] crate.
  #[error(transparent)]
  CryptoError(#[from] crypto::Error),
  /// Caused by errors from the [identity_core] crate.
  #[error(transparent)]
  CoreError(#[from] identity_core::Error),
  /// Caused by errors from the [identity_did] crate.
  #[error(transparent)]
  DIDError(#[from] identity_did::Error),
  /// Caused by errors from the [identity_credential] crate.
  #[error(transparent)]
  CredentialError(#[from] identity_credential::Error),
  /// Caused by errors from the [identity_iota] crate.
  #[error(transparent)]
  IotaError(#[from] identity_iota::Error),
  /// Caused by attempting to perform an invalid IO operation.
  #[error(transparent)]
  IoError(#[from] std::io::Error),
  /// Caused by errors from the [iota_stronghold] crate.
  #[cfg(feature = "stronghold")]
  #[error(transparent)]
  StrongholdError(#[from] iota_stronghold::Error),
  /// Caused by errors from an invalid Stronghold procedure.
  #[error("Stronghold error: {0}")]
  StrongholdResult(String),
  /// Caused by attempting to parse an invalid Stronghold resource index.
  #[error("Stronghold resource index malformed")]
  InvalidResourceIndex,
  /// Caused by attempting to access a Stronghold snapshot without a password.
  #[error("Stronghold snapshot password not found")]
  StrongholdPasswordNotSet,
  /// Caused by receiving an unexpected return value from a Stronghold procedure.
  #[error("Stronghold procedure returned unexpected type")]
  StrongholdProcedureFailure,
  /// Caused by an internal panic in the Stronghold runtime.
  #[error("Stronghold mutex poisoned: {0}")]
  StrongholdMutexPoisoned(&'static str),
  /// Caused by attempting to read a poisoned shared resource.
  #[error("Shared resource poisoned: read")]
  SharedReadPoisoned,
  /// Caused by attempting to write a poisoned shared resource.
  #[error("Shared resource poisoned: write")]
  SharedWritePoisoned,
  /// Caused by attempting to increment a generation above the maximum value.
  #[error("Generation overflow")]
  GenerationOverflow,
  /// Caused by attempting to decrement a generation below the minimum value.
  #[error("Generation underflow")]
  GenerationUnderflow,
  /// Caused by attempting to find an identity key vault that does not exist.
  #[error("Key vault not found")]
  KeyVaultNotFound,
  /// Caused by attempting to find a key in storage that does not exist.
  #[error("key not found")]
  KeyNotFound,
  /// Caused by attempting to find an identity that does not exist.
  #[error("Identity not found")]
  IdentityNotFound,
  /// Caused by attempting to perform an upate in an invalid context.
  #[error("Update Error: {0}")]
  UpdateError(#[from] crate::updates::UpdateError),
  /// Caused by providing bytes that cannot be used as a private key of the
  /// [`KeyType`][identity_core::crypto::KeyType].
  #[error("Invalid Private Key: {0}")]
  InvalidPrivateKey(String),
  /// Caused by attempting to create an account for an identity that is already managed by another account.
  #[error("Identity Is In-use")]
  IdentityInUse,
  #[error("method missing fragment")]
  MethodMissingFragment,
}

#[doc(hidden)]
pub trait PleaseDontMakeYourOwnResult<T> {
  #[allow(clippy::wrong_self_convention)]
  fn to_result(self) -> Result<T>;
}

impl From<identity_did::did::DIDError> for Error {
  fn from(error: identity_did::did::DIDError) -> Self {
    identity_did::Error::from(error).into()
  }
}
