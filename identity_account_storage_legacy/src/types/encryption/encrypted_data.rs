// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

/// The ciphertext together with supplementary data.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptedData {
  pub associated_data: Vec<u8>,
  pub nonce: Vec<u8>,
  pub tag: Vec<u8>,
  pub ciphertext: Vec<u8>,
  pub encrypted_cek: Vec<u8>,
  pub ephemeral_public_key: Vec<u8>,
}

impl EncryptedData {
  /// Creates a new `EncryptedData` instance.
  pub fn new(
    nonce: Vec<u8>,
    associated_data: Vec<u8>,
    tag: Vec<u8>,
    ciphertext: Vec<u8>,
    encrypted_cek: Vec<u8>,
    ephemeral_public_key: Vec<u8>,
  ) -> Self {
    Self {
      associated_data,
      nonce,
      tag,
      ciphertext,
      encrypted_cek,
      ephemeral_public_key,
    }
  }
}
