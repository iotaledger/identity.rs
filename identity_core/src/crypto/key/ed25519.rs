// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryInto;
use core::marker::PhantomData;
use crypto::signatures::ed25519;

use crate::error::Result;

/// An implementation of `Ed25519` signatures.
#[derive(Clone, Copy, Debug)]
pub struct Ed25519<T: ?Sized = [u8]>(PhantomData<T>);

impl Ed25519 {
  /// Length in bytes of an Ed25519 private key.
  pub const PRIVATE_KEY_LENGTH: usize = ed25519::SECRET_KEY_LENGTH;
  /// Length in bytes of an Ed25519 public key.
  pub const PUBLIC_KEY_LENGTH: usize = ed25519::PUBLIC_KEY_LENGTH;
  /// Length in bytes of an Ed25519 signature.
  pub const SIGNATURE_LENGTH: usize = ed25519::SIGNATURE_LENGTH;
}

/// Reconstructs an Ed25519 private key from a byte array.
pub(crate) fn ed25519_private_try_from_bytes(bytes: &[u8]) -> Result<ed25519::SecretKey> {
  let private_key_bytes: [u8; Ed25519::PRIVATE_KEY_LENGTH] = bytes
    .try_into()
    .map_err(|_| crate::Error::InvalidKeyLength(bytes.len(), Ed25519::PRIVATE_KEY_LENGTH))?;
  Ok(ed25519::SecretKey::from_bytes(private_key_bytes))
}

/// Reconstructs an Ed25519 public key from a byte array.
pub(crate) fn ed25519_public_try_from_bytes(bytes: &[u8]) -> Result<ed25519::PublicKey> {
  let public_key_bytes: [u8; Ed25519::PUBLIC_KEY_LENGTH] = bytes
    .try_into()
    .map_err(|_| crate::Error::InvalidKeyLength(bytes.len(), Ed25519::PUBLIC_KEY_LENGTH))?;
  ed25519::PublicKey::try_from_bytes(public_key_bytes).map_err(Into::into)
}
