// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

use uuid::Uuid;

/// An identifier for a DIDComm messaging thread.
#[derive(Debug, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ThreadId(Uuid);

impl ThreadId {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    Self(Uuid::new_v4())
  }
}

impl Display for ThreadId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
