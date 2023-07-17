// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_verification::jose::jwk::Jwk;

use super::KeyId;

/// The output of a JWK key generation.
#[non_exhaustive]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JwkGenOutput {
  /// The key identifier of the generated JWK.
  pub key_id: KeyId,
  /// The generated JWK.
  pub jwk: Jwk,
}

impl JwkGenOutput {
  /// Constructs a new JWK generation output.
  pub fn new(key_id: KeyId, jwk: Jwk) -> Self {
    Self { key_id, jwk }
  }
}
