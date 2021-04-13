// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum JweFormat {
  Compact,
  General,
  Flatten,
}

impl Default for JweFormat {
  fn default() -> Self {
    Self::Compact
  }
}
