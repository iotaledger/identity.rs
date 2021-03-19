// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;
use identity_did::verification::MethodScope;

#[derive(Clone, Debug)]
pub struct MethodConfig {
  pub(crate) type_: KeyType,
  pub(crate) fragment: Option<String>,
  pub(crate) scope: Option<MethodScope>,
}

impl MethodConfig {
  pub fn new() -> Self {
    Self {
      type_: KeyType::Ed25519,
      fragment: None,
      scope: None,
    }
  }

  #[must_use]
  pub fn type_(mut self, value: KeyType) -> Self {
    self.type_ = value;
    self
  }

  #[must_use]
  pub fn fragment<T>(mut self, value: T) -> Self
  where
    T: Into<String>,
  {
    self.fragment = Some(value.into());
    self
  }

  #[must_use]
  pub fn scope(mut self, value: MethodScope) -> Self {
    self.scope = Some(value);
    self
  }
}

impl Default for MethodConfig {
  fn default() -> Self {
    Self::new()
  }
}
