// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

use crate::types::CekAlgorithm;
use crate::types::EncryptionAlgorithm;

/// Contains the [`EncryptionAlgorithm`] and the [`CekAlgorithm`] that will be used to encrypt or decrypt data.
#[derive(Clone, Deserialize, Serialize)]
pub struct EncryptionOptions {
  encryption_algorithm: EncryptionAlgorithm,
  cek_algorithm: CekAlgorithm,
}

impl EncryptionOptions {
  /// Creates a new [`EncryptionOptions`] instance with the specified algorithms.
  pub fn new(encryption_algorithm: EncryptionAlgorithm, cek_algorithm: CekAlgorithm) -> Self {
    EncryptionOptions {
      encryption_algorithm,
      cek_algorithm,
    }
  }

  /// Returns the [`EncryptionAlgorithm`].
  pub fn encryption_algorithm(&self) -> EncryptionAlgorithm {
    self.encryption_algorithm
  }

  /// Returns the [`CekAlgorithm`].
  pub fn cek_algorithm(&self) -> &CekAlgorithm {
    &self.cek_algorithm
  }
}
