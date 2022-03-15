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
  /// Caused by attempting to perform an invalid IO operation.
  #[error(transparent)]
  IoError(#[from] std::io::Error),
  /// Caused by errors from the [iota_stronghold] crate.
  #[cfg(feature = "stronghold")]
  #[error(transparent)]
  StrongholdError(#[from] crate::stronghold::StrongholdError),

  /// Caused by attempting to increment a generation above the maximum value.
  #[error("Generation overflow")]
  GenerationOverflow,
  /// Caused by attempting to decrement a generation below the minimum value.
  #[error("Generation underflow")]
  GenerationUnderflow,
  /// Caused by providing bytes that cannot be used as a private key of the
  /// [`KeyType`][identity_core::crypto::KeyType].
  #[error("Invalid Private Key: {0}")]
  InvalidPrivateKey(String),
  /// Caused by attempting to find a key in storage that does not exist.
  #[error("key not found")]
  KeyNotFound,
  /// Caused by attempting to find an identity key vault that does not exist.
  #[error("Key vault not found")]
  KeyVaultNotFound,
  /// Caused by attempting to read a poisoned shared resource.
  #[error("Shared resource poisoned: read")]
  SharedReadPoisoned,
  /// Caused by attempting to write a poisoned shared resource.
  #[error("Shared resource poisoned: write")]
  SharedWritePoisoned,
  #[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
  #[error("JsValue serialization error: {0}")]
  SerializationError(String),
  #[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
  #[error("javascript function threw an exception: {0}")]
  JsError(String),
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
