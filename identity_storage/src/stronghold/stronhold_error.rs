// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

use identity_core::common::SingleStructError;

/// Error type for key storage operations.
pub type StrongholdError = SingleStructError<StrongholdErrorKind>;

/// The cause of the failed key storage operation.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum StrongholdErrorKind {
  Unspecified,
  MemoryError,
  SnapshotOperation,
  ClientError,
}

impl StrongholdErrorKind {
  pub const fn as_str(&self) -> &str {
    match self {
      Self::Unspecified => "unspecified error",
      Self::MemoryError => "memory error",
      Self::SnapshotOperation => "stronghold snapshot operation failed",
      Self::ClientError => "stronghold client error",
    }
  }
}
impl AsRef<str> for StrongholdErrorKind {
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

impl Display for StrongholdErrorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_str())
  }
}
