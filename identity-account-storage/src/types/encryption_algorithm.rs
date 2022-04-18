// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum EncryptionAlgorithm {
  Aes256Gcm,
}
