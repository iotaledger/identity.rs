// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub enum Digest {
  Sha256 = 1,
}

impl Default for Digest {
  fn default() -> Self {
    Self::Sha256
  }
}
