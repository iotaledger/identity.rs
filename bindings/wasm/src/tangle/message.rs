// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::DIDMessageEncoding;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = DIDMessageEncoding)]
#[derive(Copy, Clone, Debug)]
pub enum WasmDIDMessageEncoding {
  Json = 0,
  JsonBrotli = 1,
}

impl From<DIDMessageEncoding> for WasmDIDMessageEncoding {
  fn from(encoding: DIDMessageEncoding) -> Self {
    match encoding {
      DIDMessageEncoding::Json => Self::Json,
      DIDMessageEncoding::JsonBrotli => Self::JsonBrotli,
    }
  }
}

impl From<WasmDIDMessageEncoding> for DIDMessageEncoding {
  fn from(encoding: WasmDIDMessageEncoding) -> Self {
    match encoding {
      WasmDIDMessageEncoding::Json => Self::Json,
      WasmDIDMessageEncoding::JsonBrotli => Self::JsonBrotli,
    }
  }
}
