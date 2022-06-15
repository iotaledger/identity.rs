// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::ciphers::aes::Aes256Gcm;
use crypto::ciphers::traits::Aead;
use serde::Deserialize;
use serde::Serialize;

/// Supported content encryption algorithms.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum EncryptionAlgorithm {
  /// AES GCM using 256-bit key.
  AES256GCM,
}

impl EncryptionAlgorithm {
  /// Returns the length of the cipher's key.
  pub const fn key_length(&self) -> usize {
    match self {
      EncryptionAlgorithm::AES256GCM => Aes256Gcm::KEY_LENGTH,
    }
  }
}
