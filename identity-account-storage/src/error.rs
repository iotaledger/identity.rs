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
  /// Caused by errors from the [crypto] crate.
  #[error(transparent)]
  CryptoError(#[from] crypto::error::Error),
  /// Caused by errors from the [`identity_iota_core`] crate.
  #[error("DID creation failed: {0}")]
  DIDCreationError(String),
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
  /// Caused by providing bytes that cannot be used as a private key of the
  /// [`KeyType`][identity_core::crypto::KeyType].
  #[error("Invalid Private Key: {0}")]
  InvalidPrivateKey(String),
  /// Caused by attempting to perform Diffie-Hellman key exchange with the wrong key type
  #[error("Invalid Key Type: {0}")]
  InvalidKeyType(String),
  /// Caused by providing bytes that cannot be used as a public key of the
  /// [`KeyType`][identity_core::crypto::KeyType].
  #[error("Invalid Public Key: {0}")]
  InvalidPublicKey(String),
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
  /// Caused by attempting to create a DID that already exists.
  #[error("identity already exists")]
  IdentityAlreadyExists,
  #[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
  #[error("JsValue serialization error: {0}")]
  SerializationError(String),
  #[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
  #[error("javascript function threw an exception: {0}")]
  JsError(String),
}

impl From<identity_did::did::DIDError> for Error {
  fn from(error: identity_did::did::DIDError) -> Self {
    identity_did::Error::from(error).into()
  }
}
