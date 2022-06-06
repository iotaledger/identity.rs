// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::did::BitmapRevocationEndpoint;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// A parsed data url.
#[wasm_bindgen(js_name = BitmapRevocationEndpoint, inspectable)]
#[derive(Clone, Debug)]
pub struct WasmBitmapRevocationEndpoint(pub(crate) BitmapRevocationEndpoint);

#[wasm_bindgen(js_class = BitmapRevocationEndpoint)]
impl WasmBitmapRevocationEndpoint {
  /// Parses an [`BitmapRevocationEndpoint`] from the given input String.
  #[wasm_bindgen]
  pub fn parse(input: String) -> Result<WasmBitmapRevocationEndpoint> {
    Ok(WasmBitmapRevocationEndpoint(
      BitmapRevocationEndpoint::parse(&input).wasm_result()?,
    ))
  }

  /// Returns the `BitmapRevocationEndpoint` as a String.
  #[wasm_bindgen]
  pub fn into_string(&self) -> String {
    self.0.clone().into_string()
  }

  /// Returns the data from the [`BitmapRevocationEndpoint`].
  #[wasm_bindgen]
  pub fn data(&self) -> String {
    self.0.data().to_owned()
  }
}

impl From<BitmapRevocationEndpoint> for WasmBitmapRevocationEndpoint {
  fn from(endpoint: BitmapRevocationEndpoint) -> Self {
    WasmBitmapRevocationEndpoint(endpoint)
  }
}

impl From<WasmBitmapRevocationEndpoint> for BitmapRevocationEndpoint {
  fn from(endpoint: WasmBitmapRevocationEndpoint) -> Self {
    endpoint.0
  }
}
