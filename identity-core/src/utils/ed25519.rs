// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519;

pub use self::errors::Ed25519KeyPairGenerationError;
use crate::crypto::PrivateKey;
use crate::crypto::PublicKey;
mod errors {
  use thiserror::Error as DeriveError;

  /// Caused by a failure to generate an ED25519 Keypair
  #[derive(Debug, DeriveError)]
  #[error("failed to generate a ed25519 key-pair: {inner}")]
  pub struct Ed25519KeyPairGenerationError {
    #[source]
    pub(super) inner: crypto::Error,
  }
}

/// Generates a new pair of public/private ed25519 keys.
pub fn generate_ed25519_keypair() -> Result<(PublicKey, PrivateKey), Ed25519KeyPairGenerationError> {
  let secret: ed25519::SecretKey =
    ed25519::SecretKey::generate().map_err(|inner| Ed25519KeyPairGenerationError { inner })?;
  let public: ed25519::PublicKey = secret.public_key();

  let private: PrivateKey = secret.to_bytes().to_vec().into();
  let public: PublicKey = public.to_bytes().to_vec().into();

  Ok((public, private))
}

/// Generates a list of public/private ed25519 keys.
pub fn generate_ed25519_keypairs(count: usize) -> Result<Vec<(PublicKey, PrivateKey)>, Ed25519KeyPairGenerationError> {
  (0..count).map(|_| generate_ed25519_keypair()).collect()
}
