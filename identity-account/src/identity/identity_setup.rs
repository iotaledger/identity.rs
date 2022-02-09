// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;

use crate::types::MethodSecret;
/// Configuration used to create a new Identity.
/// Allows providing a pre-existing private key using `method_secret`.
#[derive(Clone, Debug)]
pub struct IdentitySetup {
  pub(crate) key_type: KeyType,
  pub(crate) method_secret: Option<MethodSecret>,
}

impl IdentitySetup {
  /// Creates a new `IdentitySetup` instance.
  pub const fn new() -> Self {
    Self {
      key_type: KeyType::Ed25519,
      method_secret: None,
    }
  }

  /// Sets the key type of the initial verification method.
  #[must_use]
  pub fn key_type(mut self, value: KeyType) -> Self {
    self.key_type = value;
    self
  }

  /// Sets the [`MethodSecret`] for the Identity creation.
  ///
  /// Note that only [`MethodSecret::Ed25519`] is currently supported.
  #[must_use]
  pub fn method_secret(mut self, value: MethodSecret) -> Self {
    self.method_secret = Some(value);
    self
  }
}

impl Default for IdentitySetup {
  fn default() -> Self {
    Self::new()
  }
}
