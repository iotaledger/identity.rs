// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum JwsFormat {
  Compact,
  General,
  Flatten,
}

impl Default for JwsFormat {
  fn default() -> Self {
    Self::Compact
  }
}
