// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone)]
pub struct DIDCommState {
  pub(crate) require_anon_crypt: bool,
}

impl DIDCommState {
  pub fn new() -> Self {
    Self {
      require_anon_crypt: true,
    }
  }
}
