// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub type IotaStrongholdResult<T> = Result<T, StrongholdError>;

/// Caused by errors from the [`iota_stronghold`] crate.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum StrongholdError {
  #[error(transparent)]
  StrongholdActorError(#[from] iota_stronghold::ActorError),
  #[error(transparent)]
  StrongholdWriteError(#[from] iota_stronghold::WriteError),
  #[error(transparent)]
  StrongholdReadError(#[from] iota_stronghold::ReadError),
  #[error(transparent)]
  StrongholdFatalEngineError(#[from] iota_stronghold::FatalEngineError),
  #[error(transparent)]
  StrongholdMailboxError(#[from] iota_stronghold::MailboxError),
  /// Caused by a failing Stronghold procedure.
  #[error("Stronghold procedure failed: {0}")]
  StrongholdProcedureFailure(#[from] iota_stronghold::procedures::ProcedureError),
  /// Caused by attempting to perform an invalid IO operation.
  #[error(transparent)]
  IoError(#[from] std::io::Error),

  /// Caused by attempting to access a Stronghold snapshot without a password.
  #[error("Stronghold snapshot password not found")]
  StrongholdPasswordNotSet,
  /// Caused by errors from an invalid Stronghold procedure.
  #[error("Stronghold error: {0}")]
  StrongholdResult(String),
  #[error("Record Error")]
  RecordError,
  /// Caused by attempting to parse an invalid Stronghold resource index.
  #[error("Stronghold resource index malformed")]
  InvalidResourceIndex,
}
