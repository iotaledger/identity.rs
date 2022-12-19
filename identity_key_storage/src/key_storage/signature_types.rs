// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

pub struct Ed25519SignatureAlgorithm;

impl Display for Ed25519SignatureAlgorithm {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Ed25519SignatureAlgorithm")
  }
}
