// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

use super::client_path::ClientPath;

pub type StrongholdResult<T> = Result<T, StrongholdError>;

/// Caused by errors from the [`iota_stronghold`] crate.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum StrongholdError {
  #[error("failed to `{0}` stronghold client `{1}` due to: {2}")]
  ClientError(ClientOperation, ClientPath, #[source] iota_stronghold::ClientError),
  #[error("store `{0}` operation failed: {1}")]
  StoreError(StoreOperation, #[source] iota_stronghold::ClientError),
  // TODO: Include operation?
  #[error("vault operation failed: {0}")]
  VaultError(#[source] iota_stronghold::ClientError),
  // TODO: Include procedure name?
  #[error("procedure failed: {0}")]
  ProcedureError(#[source] iota_stronghold::procedures::ProcedureError),
  // TODO: Include whether it was read/write?
  #[error("snapshot operation failed: {0}")]
  SnapshotError(#[source] iota_stronghold::ClientError),
}

#[derive(Debug, Clone)]
pub enum StoreOperation {
  Insert,
  Delete,
  Get,
}

impl Display for StoreOperation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      StoreOperation::Insert => f.write_str("insert"),
      StoreOperation::Delete => f.write_str("delete"),
      StoreOperation::Get => f.write_str("get"),
    }
  }
}

#[derive(Debug, Clone)]
pub enum ClientOperation {
  Load,
  Persist,
}

impl Display for ClientOperation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ClientOperation::Load => f.write_str("load"),
      ClientOperation::Persist => f.write_str("persist"),
    }
  }
}
