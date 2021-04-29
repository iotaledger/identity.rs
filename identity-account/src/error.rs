// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur when working with Identity Accounts.

/// Alias for a `Result` with the error type [`Error`].
pub type Result<T, E = Error> = ::core::result::Result<T, E>;

/// This type represents all possible errors that can occur in the library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  CryptoError(#[from] crypto::Error),
  #[error(transparent)]
  CoreError(#[from] identity_core::Error),
  #[error(transparent)]
  DIDError(#[from] identity_did::Error),
  #[error(transparent)]
  CredentialError(#[from] identity_credential::Error),
  #[error(transparent)]
  IotaError(#[from] identity_iota::Error),
  #[error(transparent)]
  IoError(#[from] std::io::Error),
  #[error(transparent)]
  ActorSystemError(#[from] riker::system::SystemError),
  #[error(transparent)]
  StrongholdError(#[from] iota_stronghold::Error),
  #[error("Stronghold error: {0}")]
  StrongholdResult(String),
  #[error("Stronghold resource index malformed")]
  InvalidResourceIndex,
  #[error("Stronghold snapshot password not found")]
  StrongholdPasswordNotSet,
  #[error("Stronghold procedure returned unexpected type")]
  StrongholdProcedureFailure,
  #[error("Stronghold mutex poisoned: {0}")]
  StrongholdMutexPoisoned(&'static str),
  #[error("Shared resource poisoned: read")]
  SharedReadPoisoned,
  #[error("Shared resource poisoned: write")]
  SharedWritePoisoned,
  #[error("Generation overflow")]
  GenerationOverflow,
  #[error("Generation underflow")]
  GenerationUnderflow,
  #[error("Too many identities")]
  IdentityIdOverflow,
  #[error("Invalid identity id")]
  IdentityIdInvalid,
  #[error("Document id not found")]
  MissingDocumentId,
  #[error("Key vault not found")]
  KeyVaultNotFound,
  #[error("Key pair not found")]
  KeyPairNotFound,
  #[error("Identity not found")]
  IdentityNotFound,
  #[error("Event not found")]
  EventNotFound,
  #[error("Identity already exists")]
  IdentityAlreadyExists,
  #[error("Verification Method not found")]
  MethodNotFound,
  #[error("Service not found")]
  ServiceNotFound,
  #[error("Command Error: {0}")]
  CommandError(#[from] crate::events::CommandError),
}

#[doc(hidden)]
pub trait PleaseDontMakeYourOwnResult<T> {
  #[allow(clippy::wrong_self_convention)]
  fn to_result(self) -> Result<T>;
}
