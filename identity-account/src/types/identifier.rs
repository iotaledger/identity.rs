// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::Index;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Identifier {
  pub(crate) ident: String,
  pub(crate) index: Index,
}

impl Identifier {
  pub const fn new(ident: String, index: Index) -> Self {
    Self { ident, index }
  }

  pub fn ident(&self) -> &str {
    &self.ident
  }

  pub fn index(&self) -> Index {
    self.index
  }
}
