// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone)]
pub struct DidCommState {
  pub(crate) require_anon_crypt: bool,
}

impl DidCommState {
  pub fn new() -> Self {
    Self {
      require_anon_crypt: true,
    }
  }
}

impl Default for DidCommState {
  fn default() -> Self {
    Self::new()
  }
}
