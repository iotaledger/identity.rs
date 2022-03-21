// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::marker::PhantomData;

use crypto::keys::x25519;

use crate::crypto::key_exchange::KeyExchange;
use crate::Error;

/// An implementation of `X25519` Diffie-Hellman cryptographic key exchange.
// TODO: this phantom data workaround sucks, maybe make key_exchange generic instead?
pub struct X25519<T: ?Sized = [u8], U: ?Sized = [u8]>(
  PhantomData<*const T>,
  PhantomData<*const U>,
);

// TODO: refactor keys to use newtype wrappers to enforce typing while avoiding
//       exporting types from pre-1.0 dependencies? Avoids the fallible runtime checks too.
//       We could just use the types from iota-crypto too if pre-1.0 is fine...
impl<T, U> KeyExchange for X25519<T, U> where
  T: AsRef<[u8]> + ?Sized,
  U: AsRef<[u8]> + ?Sized,
{
  type Private = T;
  type Public = U;
  type Output = [u8; 32];
  type Error = Error;

  fn key_exchange(private: &Self::Private, public: &Self::Public) -> core::result::Result<Self::Output, Self::Error> {
    // Copy of private key bytes will be zeroised on drop.
    let private_key: x25519::SecretKey = x25519::SecretKey::try_from_slice(private.as_ref())?;
    let public_key: x25519::PublicKey = x25519::PublicKey::try_from_slice(public.as_ref())?;
    Ok(private_key.diffie_hellman(&public_key).to_bytes())
  }
}
