// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Identifier {
  pub(crate) ident: String,
  pub(crate) index: u32,
}

impl Identifier {
  pub const fn new(ident: String, index: u32) -> Self {
    Self { ident, index }
  }

  pub fn ident(&self) -> &str {
    &self.ident
  }

  pub fn index(&self) -> u32 {
    self.index
  }
}
