// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

/// The ciphertext together with supplementary data.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptedData {
  associated_data: Vec<u8>,
  nonce: Vec<u8>,
  tag: Vec<u8>,
  ciphertext: Vec<u8>,
  encrypted_cek: Vec<u8>,
  ephemeral_public_key: Vec<u8>,
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

  /// Returns a reference to the associated data used in the encryption.
  pub fn associated_data(&self) -> &[u8] {
    &self.associated_data
  }

  /// Returns a reference to the chipertext (i.e the encrypted plaintext).
  pub fn ciphertext(&self) -> &[u8] {
    &self.ciphertext
  }

  /// Returns a reference to the tag generated in the encryption.
  pub fn tag(&self) -> &[u8] {
    &self.tag
  }

  /// Returns a reference to the nonce (unique random number) used in the encryption.
  pub fn nonce(&self) -> &[u8] {
    &self.nonce
  }

  /// Returns a reference to the encrypted content encryption key used in the encryption.
  pub fn encrypted_cek(&self) -> &[u8] {
    &self.encrypted_cek
  }

  /// Returns a reference to the ephemeral public key used for generating the shared secret.
  pub fn ephemeral_public_key(&self) -> &[u8] {
    &self.ephemeral_public_key
  }
}
