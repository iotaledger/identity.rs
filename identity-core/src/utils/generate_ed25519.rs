// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use ed25519_zebra::SigningKey;
use ed25519_zebra::VerificationKey;
use ed25519_zebra::VerificationKeyBytes;
use rand::rngs::OsRng;

use crate::crypto::PublicKey;
use crate::crypto::SecretKey;
use crate::error::Result;

/// Generates a new pair of public/secret ed25519 keys.
pub fn generate_ed25519() -> Result<(PublicKey, SecretKey)> {
  let secret: SigningKey = SigningKey::new(OsRng);
  let public: VerificationKey = (&secret).into();
  let public: VerificationKeyBytes = public.into();

  let public: PublicKey = public.as_ref().to_vec().into();
  let secret: SecretKey = secret.as_ref().to_vec().into();

  Ok((public, secret))
}

/// Generates a list of public/secret ed25519 keys.
pub fn generate_ed25519_list(count: usize) -> Result<Vec<(PublicKey, SecretKey)>> {
  (0..count).map(|_| generate_ed25519()).collect()
}
