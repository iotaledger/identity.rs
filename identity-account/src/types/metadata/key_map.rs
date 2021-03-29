// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;

use crate::types::Index;
use crate::types::KeyLocation;
use crate::types::AuthLocation;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct KeyMap {
  data: Vec<KeyEntry>,
}

impl KeyMap {
  pub const fn new() -> Self {
    Self { data: Vec::new() }
  }

  pub fn find(&self, fragment: &str) -> Option<&KeyEntry> {
    self.data.iter().rev().find(|entry| entry.fragment == fragment)
  }

  pub fn push(&mut self, entry: KeyEntry) {
    self.data.push(entry);
    self.data.sort_by_key(|entry| entry.auth.get());
  }
}

// =============================================================================
// =============================================================================

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyEntry {
  auth: Index,
  type_: KeyType,
  fragment: String,
}

impl KeyEntry {
  pub fn new(
    auth: Index,
    type_: KeyType,
    fragment: String,
  ) -> Self {
    Self { auth, type_, fragment }
  }

  pub fn auth(&self) -> Index {
    self.auth
  }

  pub fn type_(&self) -> KeyType {
    self.type_
  }

  pub fn fragment(&self) -> &str {
    &self.fragment
  }

  pub fn location(&self, chain: Index) -> KeyLocation<'_> {
    KeyLocation::new(AuthLocation::with_index(chain, self.auth), &self.fragment)
  }
}
