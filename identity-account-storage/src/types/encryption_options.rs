// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
#![allow(non_camel_case_types)]

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Deserialize, Serialize)]
pub struct EncryptionOptions {
  encryption_algorithm: EncryptionAlgorithm,
  cek_algorithm: CEKAlgorithm,
}

impl EncryptionOptions {
  pub fn new(encryption_algorithm: EncryptionAlgorithm, cek_algorithm: CEKAlgorithm) -> Self {
    EncryptionOptions {
      encryption_algorithm,
      cek_algorithm,
    }
  }

  pub fn encryption_algorithm(&self) -> EncryptionAlgorithm {
    self.encryption_algorithm
  }

  pub fn cek_algorithm(&self) -> CEKAlgorithm {
    self.cek_algorithm
  }
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum EncryptionAlgorithm {
  Aes256Gcm,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum CEKAlgorithm {
  ECDH_ES,
}
