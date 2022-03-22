// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, serde::Serialize, serde::Deserialize)]
pub struct AccountId(Uuid);

impl AccountId {
  /// Creates a new random identifier.
  pub fn random() -> Self {
    Self(Uuid::new_v4())
  }
}

impl Display for AccountId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
