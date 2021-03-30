// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::chain::ChainId;
use crate::types::Index;

#[derive(Clone, Debug, PartialEq)]
pub struct KeyLocation {
  pub(crate) chain: ChainId,
  pub(crate) auth: Index,
  pub(crate) diff: Index,
  pub(crate) fragment: String,
}

impl KeyLocation {
  pub const AUTH: &'static str = "authentication";

  pub fn auth(chain: ChainId, index: Index) -> Self {
    Self {
      chain,
      auth: index,
      diff: Index::ZERO, // authentication methods never chain on diff updates
      fragment: Self::AUTH.to_string(),
    }
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::with_capacity(12 + self.fragment.len());
    output.extend_from_slice(self.chain.as_bytes());
    output.extend_from_slice(&self.auth.to_bytes());
    output.extend_from_slice(&self.diff.to_bytes());
    output.extend_from_slice(self.fragment.as_bytes());
    output
  }
}
