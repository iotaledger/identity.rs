// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;

use crypto::keys::x25519;

use crate::crypto::KeyExchange;
use crate::Error;

/// An implementation of `X25519` Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange.
// TODO: this phantom data workaround sucks, maybe make key_exchange generic instead?
pub struct X25519<T: ?Sized = [u8], U: ?Sized = [u8]>(PhantomData<*const T>, PhantomData<*const U>);

// TODO: refactor keys to use newtype wrappers to enforce typing while avoiding
//       exporting types from pre-1.0 dependencies? Avoids the fallible runtime checks too.
//       We could just use the types from iota-crypto too if pre-1.0 is fine...
impl<T, U> KeyExchange for X25519<T, U>
where
  T: AsRef<[u8]> + ?Sized,
  U: AsRef<[u8]> + ?Sized,
{
  type Private = T;
  type Public = U;
  type Output = [u8; 32];
  type Error = Error;

  fn key_exchange(private: &Self::Private, public: &Self::Public) -> core::result::Result<Self::Output, Self::Error> {
    let private_key: x25519::SecretKey = x25519::SecretKey::try_from_slice(private.as_ref())?;
    let public_key: x25519::PublicKey = x25519::PublicKey::try_from_slice(public.as_ref())?;
    Ok(private_key.diffie_hellman(&public_key).to_bytes())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
}
