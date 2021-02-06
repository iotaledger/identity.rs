// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use zeroize::Zeroize;

use crate::crypto::KeyType;
use crate::crypto::PublicKey;
use crate::crypto::SecretKey;
use crate::error::Result;
use crate::utils::generate_ed25519;

/// A convenient type for representing a pair of cryptographic keys.
#[derive(Clone, Debug)]
pub struct KeyPair {
  type_: KeyType,
  public: PublicKey,
  secret: SecretKey,
}

impl KeyPair {
  /// Creates a new [`Ed25519`][`KeyType::Ed25519`] [`KeyPair`].
  pub fn new_ed25519() -> Result<Self> {
    let (public, secret): (PublicKey, SecretKey) = generate_ed25519()?;

    Ok(Self {
      type_: KeyType::Ed25519,
      public,
      secret,
    })
  }

  /// Returns the [`type`][`KeyType`] of the `KeyPair` object.
  pub const fn type_(&self) -> KeyType {
    self.type_
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_ed25519() {
    let keypair: KeyPair = KeyPair::new_ed25519().unwrap();
    assert_eq!(keypair.type_(), KeyType::Ed25519);
    assert_eq!(keypair.public().as_ref().len(), 32);
    assert_eq!(keypair.secret().as_ref().len(), 32);
  }
}
