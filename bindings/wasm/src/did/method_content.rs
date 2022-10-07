// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::crypto::PrivateKey;
use identity_iota::crypto::PublicKey;
use identity_storage::MethodContent;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use super::WasmMethodType1;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "MethodContent | undefined")]
  pub type OptionMethodContent;
}

// Workaround for having to deserialize `WasmMethodContent` from a Typescript interface
// TODO: remove when https://github.com/rustwasm/wasm-bindgen/pull/2677 is merged.
#[derive(Serialize, Deserialize)]
enum WasmMethodContentInner {
  Generate(WasmMethodType1),
  Private(WasmMethodType1, Vec<u8>),
  Public(WasmMethodType1, Vec<u8>),
}

#[wasm_bindgen(js_name = MethodContent, inspectable)]
#[derive(Serialize, Deserialize)]
pub struct WasmMethodContent(WasmMethodContentInner);

#[wasm_bindgen(js_class = MethodContent)]
impl WasmMethodContent {
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = Generate)]
  pub fn generate(methodType: WasmMethodType1) -> WasmMethodContent {
    Self(WasmMethodContentInner::Generate(methodType))
  }

  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = Private)]
  pub fn private(methodType: WasmMethodType1, privateKey: Vec<u8>) -> WasmMethodContent {
    Self(WasmMethodContentInner::Private(methodType, privateKey))
  }

  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = Public)]
  pub fn public(methodType: WasmMethodType1, publicKey: Vec<u8>) -> WasmMethodContent {
    Self(WasmMethodContentInner::Public(methodType, publicKey))
  }
}

impl_wasm_json!(WasmMethodContent, MethodContent);

impl From<WasmMethodContent> for MethodContent {
  fn from(content: WasmMethodContent) -> Self {
    match content.0 {
      WasmMethodContentInner::Generate(method_type) => MethodContent::Generate(method_type.0),
      WasmMethodContentInner::Private(method_type, private_key) => {
        MethodContent::Private(method_type.0, PrivateKey::from(private_key))
      }
      WasmMethodContentInner::Public(method_type, public_key) => {
        MethodContent::Public(method_type.0, PublicKey::from(public_key))
      }
    }
  }
}
