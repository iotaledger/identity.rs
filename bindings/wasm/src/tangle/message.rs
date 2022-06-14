// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::client::DIDMessageEncoding;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = DIDMessageEncoding)]
#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
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
