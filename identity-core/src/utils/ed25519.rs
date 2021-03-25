// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519;

use crate::crypto::PublicKey;
use crate::crypto::SecretKey;
use crate::error::Result;

/// Generates a new pair of public/secret ed25519 keys.
pub fn generate_ed25519_keypair() -> Result<(PublicKey, SecretKey)> {
  let secret: ed25519::SecretKey = ed25519::SecretKey::generate()?;
  let public: ed25519::PublicKey = secret.public_key();

  let secret: SecretKey = secret.to_le_bytes().to_vec().into();
  let public: PublicKey = public.to_compressed_bytes().to_vec().into();

  Ok((public, secret))
}

/// Generates a list of public/secret ed25519 keys.
pub fn generate_ed25519_keypairs(count: usize) -> Result<Vec<(PublicKey, SecretKey)>> {
  (0..count).map(|_| generate_ed25519_keypair()).collect()
}
