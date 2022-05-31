// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

/// The structure returned after encrypting data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EncryptedData {
  nonce: Vec<u8>,
  associated_data: Vec<u8>,
  tag: Vec<u8>,
  ciphertext: Vec<u8>,
  encrypted_cek: Vec<u8>,
}

impl EncryptedData {
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

  pub fn associated_data(&self) -> &[u8] {
    &self.associated_data
  }

  pub fn ciphertext(&self) -> &[u8] {
    &self.ciphertext
  }

  pub fn tag(&self) -> &[u8] {
    &self.tag
  }

  pub fn nonce(&self) -> &[u8] {
    &self.nonce
  }

  pub fn encrypted_cek(&self) -> &[u8] {
    &self.encrypted_cek
  }
}
