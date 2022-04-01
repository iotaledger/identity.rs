// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

use iota_stronghold::procedures::ProcedureError;
use iota_stronghold::ClientError;
use iota_stronghold::SnapshotPath;

use crate::types::KeyLocation;

use super::client_path::ClientPath;

pub type StrongholdResult<T> = Result<T, StrongholdError>;
pub type ProcedureName = &'static str;

/// Caused by errors from the [`iota_stronghold`] crate.
#[derive(Debug, thiserror::Error, strum::IntoStaticStr)]
pub enum StrongholdError {
  #[error("failed to `{0}` stronghold client `{1}` due to: {2}")]
  ClientError(ClientOperation, ClientPath, #[source] ClientError),
  #[error("store `{0}` operation failed: {1}")]
  StoreError(StoreOperation, #[source] ClientError),
  #[error("vault operation `{0}` failed: {1}")]
  VaultError(VaultOperation, #[source] ClientError),
  #[error("procedure `{0}` operating on locations {1:?} failed: {2}")]
  ProcedureError(ProcedureName, Vec<KeyLocation>, #[source] ProcedureError),
  // TODO: SnapshotPath should impl Display.
  #[error("snapshot operation `{0}` on path `{1:?}` failed: {2}")]
  SnapshotError(SnapshotOperation, SnapshotPath, #[source] ClientError),
  // TODO: Make #[source] and use {0} whenError trait is impl'd for inner MemoryError.
  #[error("failed to load password into key provider due to: {0:?}")]
  MemoryError(stronghold_engine::new_runtime::MemoryError),
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum ClientOperation {
  Load,
  Persist,
  Sync,
}

impl Display for ClientOperation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ClientOperation::Load => f.write_str("load"),
      ClientOperation::Persist => f.write_str("persist"),
      ClientOperation::Sync => f.write_str("sync"),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum SnapshotOperation {
  Read,
  Write,
}

impl Display for SnapshotOperation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SnapshotOperation::Read => f.write_str("read"),
      SnapshotOperation::Write => f.write_str("write"),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum VaultOperation {
  RecordExists,
  WriteSecret,
}

impl Display for VaultOperation {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      VaultOperation::RecordExists => f.write_str("record_exists"),
      VaultOperation::WriteSecret => f.write_str("write_secret"),
    }
  }
}
