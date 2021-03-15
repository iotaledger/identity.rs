// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;

#[derive(Clone, Debug)]
pub struct IdentityConfig {
  pub(crate) key_type: KeyType,
  pub(crate) name: Option<String>,
  pub(crate) network: Option<String>,
  pub(crate) shard: Option<String>,
  pub(crate) persist: bool,
  pub(crate) publish: bool,
}

impl IdentityConfig {
  pub const fn new() -> Self {
    Self {
      key_type: KeyType::Ed25519,
      name: None,
      network: None,
      shard: None,
      persist: true,
      publish: false, // TODO: true
    }
  }

  pub fn key_type(mut self, value: KeyType) -> Self {
    self.key_type = value;
    self
  }

  pub fn name<T>(mut self, value: T) -> Self
  where
    T: Into<String>,
  {
    self.name = Some(value.into());
    self
  }

  pub fn network<T>(mut self, value: T) -> Self
  where
    T: Into<String>,
  {
    self.network = Some(value.into());
    self
  }

  pub fn shard<T>(mut self, value: T) -> Self
  where
    T: Into<String>,
  {
    self.shard = Some(value.into());
    self
  }

  pub fn publish(mut self, value: bool) -> Self {
    self.publish = value;
    self
  }

  pub fn persist(mut self, value: bool) -> Self {
    self.persist = value;
    self
  }
}

impl Default for IdentityConfig {
  fn default() -> Self {
    Self::new()
  }
}
