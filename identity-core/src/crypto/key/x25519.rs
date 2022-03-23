// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::keys::x25519;
use crypto::signatures::ed25519;

use crate::crypto::PrivateKey;
use crate::crypto::PublicKey;
use crate::utils::ed25519_private_try_from_bytes;
use crate::utils::ed25519_public_try_from_bytes;
use crate::Result;

/// An implementation of `X25519` Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange.
pub struct X25519;

impl X25519 {
  /// Performs a cryptographic key exchange process (e.g. Diffie-Hellman) using the private key
  /// of the first party with the public key of the second party, resulting in a shared secret.
  pub fn key_exchange<PRV, PUB>(private: &PRV, public: &PUB) -> Result<[u8; 32]>
  where
    PRV: AsRef<[u8]> + ?Sized,
    PUB: AsRef<[u8]> + ?Sized,
  {
    let private_key: x25519::SecretKey = x25519::SecretKey::try_from_slice(private.as_ref())?;
    let public_key: x25519::PublicKey = x25519::PublicKey::try_from_slice(public.as_ref())?;
    Ok(private_key.diffie_hellman(&public_key).to_bytes())
  }

  /// Transforms an [`Ed25519`](crate::crypto::KeyType::Ed25519) private key to an
  /// [`X25519`](crate::crypto::KeyType::X25519) private key.
  ///
  /// This is possible because Ed25519 is birationally equivalent to Curve25519 used by X25519.
  pub fn ed25519_to_x25519_private<PRV>(private_key: &PRV) -> Result<PrivateKey>
  where
    PRV: AsRef<[u8]> + ?Sized,
  {
    let ed25519_private: ed25519::SecretKey = ed25519_private_try_from_bytes(private_key.as_ref())?;
    let x25519_private: x25519::SecretKey = x25519::SecretKey::from(&ed25519_private);
    Ok(PrivateKey::from(x25519_private.to_bytes().to_vec()))
  }

  /// Transforms an [`Ed25519`](crate::crypto::KeyType::Ed25519) public key to an
  /// [`X25519`](crate::crypto::KeyType::X25519) public key.
  ///
  /// This is possible because Ed25519 is birationally equivalent to Curve25519 used by X25519.
  pub fn ed25519_to_x25519_public<PUB>(public_key: &PUB) -> Result<PublicKey>
  where
    PUB: AsRef<[u8]> + ?Sized,
  {
    let ed25519_public: ed25519::PublicKey = ed25519_public_try_from_bytes(public_key.as_ref())?;
    let x25519_public: x25519::PublicKey = x25519::PublicKey::try_from(&ed25519_public)?;
    Ok(PublicKey::from(x25519_public.to_bytes().to_vec()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::crypto::KeyPair;
  use crate::crypto::KeyType;

  #[test]
  fn test_x25519_test_vector() {
    // The following test vector is taken from [Section 6.1 RFC 7748](https://datatracker.ietf.org/doc/html/rfc7748#section-6.1)
    let alice_secret_key: Vec<u8> =
      hex::decode("77076d0a7318a57d3c16c17251b26645df4c2f87ebc0992ab177fba51db92c2a").unwrap();
    let alice_public_key: Vec<u8> =
      hex::decode("8520f0098930a754748b7ddcb43ef75a0dbf3a0d26381af4eba4a98eaa9b4e6a").unwrap();
    let bob_secret_key: Vec<u8> =
      hex::decode("5dab087e624a8a4b79e17f8b83800ee66f3bb1292618b6fd1c2f8b27ff88e0eb").unwrap();
    let bob_public_key: Vec<u8> =
      hex::decode("de9edb7d7b7dc1b4d35b61c2ece435373f8343c85b78674dadfc7e146f882b4f").unwrap();

    let alice_secret: [u8; 32] = X25519::key_exchange(&alice_secret_key, &bob_public_key).unwrap();
    let bob_secret: [u8; 32] = X25519::key_exchange(&bob_secret_key, &alice_public_key).unwrap();
    assert_eq!(alice_secret, bob_secret);

    let expected_secret_hex: &str = "4a5d9d5ba4ce2de1728e3bf480350f25e07e21c947d19e3376f09b3c1e161742";
    assert_eq!(hex::encode(alice_secret), expected_secret_hex);
    assert_eq!(hex::encode(bob_secret), expected_secret_hex);
  }

  #[test]
  fn test_ed25519_to_x25519() {
    // Convert an Ed25519 private key to an X25519 private key.
    let ed25519: KeyPair = KeyPair::try_from_private_key_bytes(KeyType::Ed25519, &[1u8; 32]).unwrap();
    let x25519_private: PrivateKey = X25519::ed25519_to_x25519_private(&ed25519.private()).unwrap();

    // Ensure public key conversion matches derived public key.
    let x25519_public: PublicKey = X25519::ed25519_to_x25519_public(&ed25519.public()).unwrap();
    let x25519_key_pair: KeyPair =
      KeyPair::try_from_private_key_bytes(KeyType::X25519, x25519_private.as_ref()).unwrap();
    assert_eq!(x25519_public.as_ref(), x25519_key_pair.public().as_ref());

    // Ensure Diffie-Hellman key-exchange works with another X25519 key pair.
    let x25519_bob: KeyPair = KeyPair::try_from_private_key_bytes(KeyType::X25519, &[2u8; 32]).unwrap();

    let secret_key_alice: [u8; 32] = X25519::key_exchange(&x25519_private, x25519_bob.public()).unwrap();
    let secret_key_bob: [u8; 32] = X25519::key_exchange(&x25519_bob.private(), &x25519_public).unwrap();
    assert_eq!(secret_key_alice, secret_key_bob);
  }
}
