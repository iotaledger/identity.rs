// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::Index;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Identifier {
  pub(crate) index: Index,
  pub(crate) ident: String,
}

impl Identifier {
  pub const fn new(ident: String, index: Index) -> Self {
    Self { index, ident }
  }

  pub fn index(&self) -> Index {
    self.index
  }

  pub fn ident(&self) -> &str {
    &self.ident
  }
}
