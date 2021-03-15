// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type Result<T, E = Error> = ::core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
  CryptoError(crypto::Error),
  CoreError(identity_core::Error),
  DIDError(identity_did::Error),
  IotaError(identity_iota::Error),
  IoError(std::io::Error),
  ActorSystemError(riker::system::SystemError),
  StrongholdError(iota_stronghold::Error),
  StrongholdResult(String),
  InvalidResourceIndex,
  StrongholdPasswordNotSet,
  StrongholdProcedureFailure,
  StrongholdInvalidAddress,
  MutexPoisoned,
  RwLockReadPoisoned,
  RwLockWritePoisoned,
  MissingStorageAdapter,
  IdentityNotFound,
  MetadataNotFound,
}

impl From<crypto::Error> for Error {
  fn from(other: crypto::Error) -> Self {
    Self::CryptoError(other)
  }
}

impl From<identity_core::Error> for Error {
  fn from(other: identity_core::Error) -> Self {
    Self::CoreError(other)
  }
}

impl From<identity_did::Error> for Error {
  fn from(other: identity_did::Error) -> Self {
    Self::DIDError(other)
  }
}

impl From<identity_iota::Error> for Error {
  fn from(other: identity_iota::Error) -> Self {
    Self::IotaError(other)
  }
}

impl From<std::io::Error> for Error {
  fn from(other: std::io::Error) -> Self {
    Self::IoError(other)
  }
}

impl From<riker::system::SystemError> for Error {
  fn from(other: riker::system::SystemError) -> Self {
    Self::ActorSystemError(other)
  }
}

impl From<iota_stronghold::Error> for Error {
  fn from(other: iota_stronghold::Error) -> Self {
    Self::StrongholdError(other)
  }
}

#[doc(hidden)]
pub trait PleaseDontMakeYourOwnResult<T> {
  #[allow(clippy::wrong_self_convention)]
  fn to_result(self) -> Result<T>;
}
