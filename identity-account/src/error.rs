// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type Result<T, E = Error> = ::core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
  IoError(std::io::Error),
  ActorSystemError(riker::system::SystemError),
  StrongholdError(iota_stronghold::Error),
  StrongholdResult(String),
  InvalidResourceIndex,
  StrongholdPasswordNotSet,
  StrongholdProcedureFailure,
  StrongholdInvalidAddress,
  MutexPoisoned,
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
