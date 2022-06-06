// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

/// Supported content encryption algorithms.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum EncryptionAlgorithm {
  /// AES GCM using 256-bit key.
  AES256GCM,
}
