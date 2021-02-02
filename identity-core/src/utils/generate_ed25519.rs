// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use ed25519_zebra::SigningKey;
use ed25519_zebra::VerificationKey;
use ed25519_zebra::VerificationKeyBytes;
use rand::rngs::OsRng;

use crate::crypto::KeyPair;
use crate::crypto::PublicKey;
use crate::crypto::SecretKey;
use crate::error::Result;

/// Generates a new ed25519 [`KeyPair`].
pub fn generate_ed25519() -> Result<KeyPair> {
  let secret: SigningKey = SigningKey::new(OsRng);
  let public: VerificationKey = (&secret).into();
  let public: VerificationKeyBytes = public.into();

  let public: PublicKey = public.as_ref().to_vec().into();
  let secret: SecretKey = secret.as_ref().to_vec().into();

  Ok(KeyPair::new(public, secret))
}
