// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Errors that may occur when working with Identity Accounts.

/// Alias for a `Result` with the error type [`Error`].
pub type Result<T, E = Error> = ::core::result::Result<T, E>;

/// This type represents all possible errors that can occur in the library.
#[derive(Debug, thiserror::Error, flat_enum::derive::FlatEnum)]
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

// #[cfg(feature = "serde-errors")]
// #[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
// #[repr(C)]
// pub struct SerdeError {
//   pub code: ErrorCode,
//   pub description: Option<String>,
// }
//
// #[cfg(feature = "serde-errors")]
// impl SerdeError {
//   pub fn new(code: ErrorCode, description: Option<String>) -> Self {
//     Self {
//       code,
//       description,
//     }
//   }
// }
//
// #[cfg(feature = "serde-errors")]
// #[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
// #[repr(C)]
// pub enum ErrorCode {
//   /// [Error::CryptoError]
//   CryptoError,
//   /// [Error::CoreError]
//   CoreError,
//   /// [Error::DIDError]
//   DIDError,
//   /// [Error::CredentialError]
//   CredentialError,
//   /// [Error::IotaError]
//   IotaError,
//   /// [Error::IoError]
//   IoError,
//   /// [Error::ActorSystemError]
//   ActorSystemError,
//   /// [Error::StrongholdError]
//   StrongholdError,
//   /// [Error::StrongholdResult]
//   StrongholdResult,
//   /// [Error::InvalidResourceIndex]
//   InvalidResourceIndex,
//   /// [Error::StrongholdPasswordNotSet]
//   StrongholdPasswordNotSet,
//   /// [Error::StrongholdProcedureFailure]
//   StrongholdProcedureFailure,
//   /// [Error::StrongholdMutexPoisoned]
//   StrongholdMutexPoisoned,
//   /// [Error::SharedReadPoisoned]
//   SharedReadPoisoned,
//   /// [Error::SharedWritePoisoned]
//   SharedWritePoisoned,
//   /// [Error::GenerationOverflow]
//   GenerationOverflow,
//   /// [Error::GenerationUnderflow]
//   GenerationUnderflow,
//   /// [Error::IdentityIdOverflow]
//   IdentityIdOverflow,
//   /// [Error::IdentityIdInvalid]
//   IdentityIdInvalid,
//   /// [Error::MissingDocumentId]
//   MissingDocumentId,
//   /// [Error::KeyVaultNotFound]
//   KeyVaultNotFound,
//   /// [Error::KeyPairNotFound]
//   KeyPairNotFound,
//   /// [Error::IdentityNotFound]
//   IdentityNotFound,
//   /// [Error::EventNotFound]
//   EventNotFound,
//   /// [Error::IdentityAlreadyExists]
//   IdentityAlreadyExists,
//   /// [Error::MethodNotFound]
//   MethodNotFound,
//   /// [Error::ServiceNotFound]
//   ServiceNotFound,
//   /// [Error::CommandError]
//   CommandError,
// }
//
// #[cfg(feature = "serde-errors")]
// impl From<Error> for SerdeError {
//   fn from(error: Error) -> Self {
//     match error {
//       Error::CryptoError(inner) => Self::new(ErrorCode::CryptoError, Some(inner.to_string())),
//       Error::CoreError(inner) => Self::new(ErrorCode::CoreError, Some(inner.to_string())),
//       Error::DIDError(inner) => Self::new(ErrorCode::CoreError, Some(inner.to_string())),
//       Error::CredentialError(inner) => Self::new(ErrorCode::CredentialError, Some(inner.to_string())),
//       Error::IotaError(inner) => Self::new(ErrorCode::IotaError, Some(inner.to_string())),
//       Error::IoError(inner) => Self::new(ErrorCode::IoError, Some(inner.to_string())),
//       Error::ActorSystemError(inner) => Self::new(ErrorCode::ActorSystemError, Some(inner.to_string())),
//       Error::StrongholdError(inner) => Self::new(ErrorCode::StrongholdError, Some(inner.to_string())),
//       Error::StrongholdResult(inner) => Self::new(ErrorCode::StrongholdResult, Some(inner.to_string())),
//       Error::InvalidResourceIndex => Self::new(ErrorCode::InvalidResourceIndex, None),
//       Error::StrongholdPasswordNotSet => Self::new(ErrorCode::StrongholdPasswordNotSet, None),
//       Error::StrongholdProcedureFailure => Self::new(ErrorCode::StrongholdProcedureFailure, None),
//       Error::StrongholdMutexPoisoned(inner) => Self::new(ErrorCode::StrongholdMutexPoisoned,
// Some(inner.to_string())),       Error::SharedReadPoisoned => Self::new(ErrorCode::SharedReadPoisoned, None),
//       Error::SharedWritePoisoned => Self::new(ErrorCode::SharedWritePoisoned, None),
//       Error::GenerationOverflow => Self::new(ErrorCode::GenerationOverflow, None),
//       Error::GenerationUnderflow => Self::new(ErrorCode::GenerationUnderflow, None),
//       Error::IdentityIdOverflow => Self::new(ErrorCode::IdentityIdOverflow, None),
//       Error::IdentityIdInvalid => Self::new(ErrorCode::IdentityIdInvalid, None),
//       Error::MissingDocumentId => Self::new(ErrorCode::MissingDocumentId, None),
//       Error::KeyVaultNotFound => Self::new(ErrorCode::KeyVaultNotFound, None),
//       Error::KeyPairNotFound => Self::new(ErrorCode::KeyPairNotFound, None),
//       Error::IdentityNotFound => Self::new(ErrorCode::IdentityNotFound, None),
//       Error::EventNotFound => Self::new(ErrorCode::EventNotFound, None),
//       Error::IdentityAlreadyExists => Self::new(ErrorCode::IdentityAlreadyExists, None),
//       Error::MethodNotFound => Self::new(ErrorCode::MethodNotFound, None),
//       Error::ServiceNotFound => Self::new(ErrorCode::ServiceNotFound, None),
//       Error::CommandError(inner) => Self::new(ErrorCode::CommandError, Some(inner.to_string())),
//     }
//   }
// }
