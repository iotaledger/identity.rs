// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use zeroize::Zeroize;

use crate::crypto::PublicKey;
use crate::crypto::SecretKey;

/// A convenience for storing a pair of cryptographic keys
#[derive(Clone, Debug)]
pub struct KeyPair {
  public: PublicKey,
  secret: SecretKey,
}

impl KeyPair {
  /// Creates a new [`KeyPair`] from the given keys.
  pub const fn new(public: PublicKey, secret: SecretKey) -> Self {
    Self { public, secret }
  }

  /// Returns a reference to the [`PublicKey`] object.
  pub const fn public(&self) -> &PublicKey {
    &self.public
  }

  /// Returns a reference to the [`SecretKey`] object.
  pub const fn secret(&self) -> &SecretKey {
    &self.secret
  }
}

impl Drop for KeyPair {
  fn drop(&mut self) {
    self.public.zeroize();
    self.secret.zeroize();
  }
}

impl Zeroize for KeyPair {
  fn zeroize(&mut self) {
    self.public.zeroize();
    self.secret.zeroize();
  }
}
