// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur when working with Identity Accounts.

/// Alias for a `Result` with the error type [`Error`].
pub type Result<T, E = Error> = ::core::result::Result<T, E>;

/// This type represents all possible errors that can occur in the library.
#[derive(Debug, thiserror::Error)]
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
  /// Caused by an internal failure of the riker actor system.
  #[cfg(feature = "stronghold")]
  #[error(transparent)]
  ActorSystemError(#[from] riker::system::SystemError),
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
  /// Caused by attempting to add a new identity when an account is at capacity.
  #[error("Too many identities")]
  IdentityIdOverflow,
  /// Caused by attempting to parse an invalid identity id.
  #[error("Invalid identity id")]
  IdentityIdInvalid,
  /// Caused by attempting to read a DID from an unintialized identity state.
  #[error("Document id not found")]
  MissingDocumentId,
  /// Caused by attempting to find an identity key vault that does not exist.
  #[error("Key vault not found")]
  KeyVaultNotFound,
  /// Caused by attempting to find an identity key pair that does not exist.
  #[error("Key pair not found")]
  KeyPairNotFound,
  /// Caused by attempting to find an identity that does not exist.
  #[error("Identity not found")]
  IdentityNotFound,
  /// Caused by attempting to find an identity event that does not exist.
  #[error("Event not found")]
  EventNotFound,
  /// Caused by attempting to re-initialize an existing identity.
  #[error("Identity already exists")]
  IdentityAlreadyExists,
  /// Caused by attempting to find a verification method that does not exist.
  #[error("Verification Method not found")]
  MethodNotFound,
  /// Caused by attempting to find a service that does not exist.
  #[error("Service not found")]
  ServiceNotFound,
  /// Caused by attempting to perform a command in an invalid context.
  #[error("Command Error: {0}")]
  CommandError(#[from] crate::events::CommandError),
}

#[doc(hidden)]
pub trait PleaseDontMakeYourOwnResult<T> {
  #[allow(clippy::wrong_self_convention)]
  fn to_result(self) -> Result<T>;
}
