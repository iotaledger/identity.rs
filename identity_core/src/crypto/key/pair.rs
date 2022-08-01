// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::convert::TryInto;

use crypto::keys::x25519;
use crypto::signatures::ed25519;
use zeroize::Zeroize;

use crate::crypto::key::ed25519::ed25519_private_try_from_bytes;
use crate::crypto::key::secp256k1::secp256k1_private_try_from_bytes;
use crate::crypto::key::secp256k1::Secp256k1;
use crate::crypto::KeyType;
use crate::crypto::PrivateKey;
use crate::crypto::PublicKey;
use crate::crypto::X25519;
use crate::error::Result;

/// A convenient type for representing a pair of cryptographic keys.
// TODO: refactor with exact types for each key type? E.g. Ed25519KeyPair, X25519KeyPair etc.
//       Maybe a KeyPair trait with associated types? Might need typed key structs
//       like Ed25519Public, X25519Private etc. to avoid exposing pre-1.0 dependency types.
#[derive(Clone, Debug)]
pub struct KeyPair {
  type_: KeyType,
  public: PublicKey,
  private: PrivateKey,
}

impl KeyPair {
  /// Creates a new [`KeyPair`] with the given [`key type`][`KeyType`].
  pub fn new(type_: KeyType) -> Result<Self> {
    let (public, private): (PublicKey, PrivateKey) = match type_ {
      KeyType::Ed25519 => {
        let secret: ed25519::SecretKey = ed25519::SecretKey::generate()?;
        let public: ed25519::PublicKey = secret.public_key();

        let private: PrivateKey = secret.to_bytes().to_vec().into();
        let public: PublicKey = public.to_bytes().to_vec().into();

        (public, private)
      }
      KeyType::X25519 => {
        let secret: x25519::SecretKey = x25519::SecretKey::generate()?;
        let public: x25519::PublicKey = secret.public_key();

        let private: PrivateKey = secret.to_bytes().to_vec().into();
        let public: PublicKey = public.to_bytes().to_vec().into();
        (public, private)
      }
      KeyType::Secp256k1 => {
        let mut bs = [0u8; Secp256k1::PRIVATE_KEY_LENGTH];
        crypto::utils::rand::fill(&mut bs)?;
        let sk: libsecp256k1::SecretKey = secp256k1_private_try_from_bytes(&bs)?;
        let pk: libsecp256k1::PublicKey = libsecp256k1::PublicKey::from_secret_key(&sk);

        let private: PrivateKey = sk.serialize().to_vec().into();
        let public: PublicKey = pk.serialize_compressed().to_vec().into();
        (public, private)
      }
    };

    Ok(Self { type_, public, private })
  }

  /// Reconstructs a [`KeyPair`] from the bytes of a private key.
  ///
  /// The private key for [`Ed25519`][`KeyType::Ed25519`] must be a 32-byte seed in compliance
  /// with [RFC 8032](https://datatracker.ietf.org/doc/html/rfc8032#section-3.2).
  /// Other implementations often use another format. See [this blog post](https://blog.mozilla.org/warner/2011/11/29/ed25519-keys/) for further explanation.
  pub fn try_from_private_key_bytes(key_type: KeyType, private_key_bytes: &[u8]) -> Result<Self> {
    let (public, private) = match key_type {
      KeyType::Ed25519 => {
        let private_key: ed25519::SecretKey = ed25519_private_try_from_bytes(private_key_bytes)?;
        let public_key: ed25519::PublicKey = private_key.public_key();

        let private: PrivateKey = private_key.to_bytes().to_vec().into();
        let public: PublicKey = public_key.to_bytes().to_vec().into();
        (public, private)
      }
      KeyType::X25519 => {
        let private_key_bytes: [u8; X25519::PRIVATE_KEY_LENGTH] = private_key_bytes
          .try_into()
          .map_err(|_| crate::Error::InvalidKeyLength(private_key_bytes.len(), X25519::PRIVATE_KEY_LENGTH))?;
        let private_key: x25519::SecretKey = x25519::SecretKey::from_bytes(private_key_bytes);
        let public_key: x25519::PublicKey = private_key.public_key();

        let private: PrivateKey = private_key.to_bytes().to_vec().into();
        let public: PublicKey = public_key.to_bytes().to_vec().into();
        (public, private)
      }
      KeyType::Secp256k1 => {
        let sk: libsecp256k1::SecretKey = secp256k1_private_try_from_bytes(private_key_bytes)?;
        let pk: libsecp256k1::PublicKey = libsecp256k1::PublicKey::from_secret_key(&sk);

        let private: PrivateKey = sk.serialize().to_vec().into();
        let public: PublicKey = pk.serialize_compressed().to_vec().into();
        (public, private)
      }
    };

    Ok(Self {
      type_: key_type,
      public,
      private,
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

  /// Returns a reference to the [`PrivateKey`] object.
  pub const fn private(&self) -> &PrivateKey {
    &self.private
  }
}

impl Drop for KeyPair {
  fn drop(&mut self) {
    self.public.zeroize();
    self.private.zeroize();
  }
}

impl Zeroize for KeyPair {
  fn zeroize(&mut self) {
    self.public.zeroize();
    self.private.zeroize();
  }
}

impl From<(KeyType, PublicKey, PrivateKey)> for KeyPair {
  fn from(other: (KeyType, PublicKey, PrivateKey)) -> Self {
    Self {
      type_: other.0,
      public: other.1,
      private: other.2,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new_ed25519() {
    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    assert_eq!(keypair.type_(), KeyType::Ed25519);
    assert_eq!(keypair.public().as_ref().len(), 32);
    assert_eq!(keypair.private().as_ref().len(), 32);
  }

  #[test]
  fn test_new_x25519() {
    let keypair: KeyPair = KeyPair::new(KeyType::X25519).unwrap();
    assert_eq!(keypair.type_(), KeyType::X25519);
    assert_eq!(keypair.public().as_ref().len(), 32);
    assert_eq!(keypair.private().as_ref().len(), 32);
  }

  #[test]
  fn test_new_secp256k1() {
    let keypair: KeyPair = KeyPair::new(KeyType::Secp256k1).unwrap();
    assert_eq!(keypair.type_(), KeyType::Secp256k1);
    assert_eq!(keypair.public().as_ref().len(), 33);
    assert_eq!(keypair.private().as_ref().len(), 32);
  }

  #[test]
  fn test_try_from_private_key_bytes() {
    for key_type in [KeyType::Ed25519, KeyType::X25519, KeyType::Secp256k1] {
      let keypair: KeyPair = KeyPair::new(key_type).unwrap();
      let reconstructed: KeyPair = KeyPair::try_from_private_key_bytes(key_type, keypair.private.as_ref()).unwrap();
      assert_eq!(keypair.private.as_ref(), reconstructed.private.as_ref());
      assert_eq!(keypair.public.as_ref(), reconstructed.public.as_ref());
    }
  }
}
