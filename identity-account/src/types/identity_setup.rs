// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::PrivateKey;

/// Configuration used to create a new Identity.
#[derive(Clone, Debug)]
pub struct IdentitySetup {
  /// Use a pre-generated Ed25519 private key for the DID.
  pub(crate) private_key: Option<PrivateKey>,
}

impl IdentitySetup {
  /// Creates a new `IdentitySetup` instance.
  pub const fn new() -> Self {
    Self { private_key: None }
  }

  /// Sets the Ed25519 private key to use for Identity creation.
  #[must_use]
  pub fn private_key(mut self, value: PrivateKey) -> Self {
    self.private_key = Some(value);
    self
  }
}

impl Default for IdentitySetup {
  fn default() -> Self {
    Self::new()
  }
}
