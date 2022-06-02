// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

/// The structure returned after encrypting data.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptedData {
  nonce: Vec<u8>,
  associated_data: Vec<u8>,
  tag: Vec<u8>,
  ciphertext: Vec<u8>,
  encrypted_cek: Vec<u8>,
}

impl EncryptedData {
  /// Creates a new `EncryptedData` instance.
  pub fn new(
    nonce: Vec<u8>,
    associated_data: Vec<u8>,
    tag: Vec<u8>,
    ciphertext: Vec<u8>,
    encrypted_cek: Vec<u8>,
  ) -> Self {
    Self {
      nonce,
      associated_data,
      tag,
      ciphertext,
      encrypted_cek,
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
}
