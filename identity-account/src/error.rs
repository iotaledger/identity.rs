// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::events::CommandError;

pub type Result<T, E = Error> = ::core::result::Result<T, E>;

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
  #[error("Failed to parse integer: {0}")]
  InvalidIntegerBytes(&'static str),
  #[error("Failed to increment index: overflow")]
  IndexOverflow,
  #[error("Failed to increment index: underflow")]
  IndexUnderflow,
  #[error("Failed to parse chain id")]
  InvalidChainId,
  #[error("Failed to parse chain key")]
  InvalidChainKey,
  #[error("Failed to parse chain slot")]
  InvalidChainSlot,
  #[error("Chain document not found")]
  MissingChainDocument,
  #[error("Chain key vault not found")]
  KeyVaultNotFound,
  #[error("Chain key pair not found")]
  KeyPairNotFound,
  #[error("Chain id not found")]
  ChainIdNotFound,
  #[error("Chain event not found")]
  ChainEventNotFound,
  #[error("Chain already exists")]
  ChainAlreadyExists,
  #[error("No chains found")]
  NoChainsFound,
  #[error("Diff message id not found")]
  DiffMessageIdNotFound,
  #[error("Auth message id not found")]
  AuthMessageIdNotFound,
  #[error("Verification method not found")]
  MethodNotFound,
  #[error("Service not found")]
  ServiceNotFound,
  #[error("Command Error: {0}")]
  CommandError(#[from] CommandError),
}

#[doc(hidden)]
pub trait PleaseDontMakeYourOwnResult<T> {
  #[allow(clippy::wrong_self_convention)]
  fn to_result(self) -> Result<T>;
}
