// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;

pub struct Ed25519KeyType;

impl Display for Ed25519KeyType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Ed25519KeyType")
  }
}

pub struct X25519KeyType;

impl Display for X25519KeyType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("X25519KeyType")
  }
}
