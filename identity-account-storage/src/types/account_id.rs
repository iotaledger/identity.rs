// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

use uuid::Uuid;

/// A unique storage identifier for an account.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, serde::Serialize, serde::Deserialize)]
pub struct AccountId(Uuid);

impl AccountId {
  /// Generates a new random identifier.
  pub fn generate() -> Self {
    Self(Uuid::new_v4())
  }

  /// Returns the slice of bytes making up the identifier.
  pub fn as_bytes(&self) -> &[u8] {
    self.0.as_bytes()
  }
}

impl Display for AccountId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<[u8; 16]> for AccountId {
  fn from(bytes: [u8; 16]) -> Self {
    Self(Uuid::from_bytes(bytes))
  }
}
