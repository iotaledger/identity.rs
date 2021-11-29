// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519;

use crate::crypto::PrivateKey;
use crate::crypto::PublicKey;

/// Generates a new pair of public/private Ed25519 keys.
///
/// Note that the private key is a 32-byte seed in compliance with [RFC 8032](https://datatracker.ietf.org/doc/html/rfc8032#section-3.2).
/// Other implementations often use another format. See [this blog post](https://blog.mozilla.org/warner/2011/11/29/ed25519-keys/) for further explanation.
pub fn generate_ed25519_keypair() -> Result<(PublicKey, PrivateKey), Ed25519KeyPairGenerationError> {
  let secret: ed25519::SecretKey =
    ed25519::SecretKey::generate().map_err(|inner| Ed25519KeyPairGenerationError { inner })?;
  let public: ed25519::PublicKey = secret.public_key();

  let private: PrivateKey = secret.to_bytes().to_vec().into();
  let public: PublicKey = public.to_bytes().to_vec().into();

  Ok((public, private))
}

// Reconstructs a pair of public/private Ed25519 keys from an ed25519::SecretKey.
pub(crate) fn keypair_from_ed25519_private_key(private_key: ed25519::SecretKey) -> (PublicKey, PrivateKey) {
  let public: ed25519::PublicKey = private_key.public_key();

  let private: PrivateKey = private_key.to_bytes().to_vec().into();
  let public: PublicKey = public.to_bytes().to_vec().into();

  (public, private)
}

/// Generates a list of public/private Ed25519 keys.
///
/// See [`generate_ed25519_keypair`].
pub fn generate_ed25519_keypairs(count: usize) -> Result<Vec<(PublicKey, PrivateKey)>, Ed25519KeyPairGenerationError> {
  (0..count).map(|_| generate_ed25519_keypair()).collect()
}

/// Caused by a failure to generate an ED25519 Keypair
#[derive(Debug, thiserror::Error)]
#[error("failed to generate a ed25519 key-pair: {inner}")]
pub struct Ed25519KeyPairGenerationError {
  #[source]
  pub(crate) inner: crypto::Error,
}
