// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::crypto::PrivateKey;
use identity_iota::crypto::PublicKey;
use identity_storage::MethodContent;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "MethodContent | undefined")]
  pub type OptionMethodContent;
}

// Workaround for having to deserialize `WasmMethodContent` from a Typescript interface
// TODO: remove when https://github.com/rustwasm/wasm-bindgen/pull/2677 is merged.
#[derive(Serialize, Deserialize)]
enum WasmMethodContentInner {
  Generate,
  Private(Vec<u8>),
  Public(Vec<u8>),
}

#[wasm_bindgen(js_name = MethodContent, inspectable)]
#[derive(Serialize, Deserialize)]
pub struct WasmMethodContent(WasmMethodContentInner);

#[wasm_bindgen(js_class = MethodContent)]
impl WasmMethodContent {
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = Generate)]
  pub fn generate() -> WasmMethodContent {
    Self(WasmMethodContentInner::Generate)
  }

  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = Private)]
  pub fn private(privateKey: Vec<u8>) -> WasmMethodContent {
    Self(WasmMethodContentInner::Private(privateKey))
  }

  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = Public)]
  pub fn public(publicKey: Vec<u8>) -> WasmMethodContent {
    Self(WasmMethodContentInner::Public(publicKey))
  }

  #[wasm_bindgen(js_name = isGenerate)]
  pub fn is_generate(&self) -> bool {
    matches!(self.0, WasmMethodContentInner::Generate)
  }

  #[wasm_bindgen(js_name = privateKey, getter)]
  pub fn private_key(&mut self) -> Option<Vec<u8>> {
    match &mut self.0 {
      WasmMethodContentInner::Private(ref mut private_key) => {
        let mut swap = Vec::new();
        std::mem::swap(&mut swap, private_key);
        Some(swap)
      }
      _ => None,
    }
  }

  #[wasm_bindgen(js_name = publicKey, getter)]
  pub fn public_key(&mut self) -> Option<Vec<u8>> {
    match &mut self.0 {
      WasmMethodContentInner::Public(ref mut public_key) => {
        let mut swap = Vec::new();
        std::mem::swap(&mut swap, public_key);
        Some(swap)
      }
      _ => None,
    }
  }
}

impl_wasm_json!(WasmMethodContent, MethodContent);

impl From<WasmMethodContent> for MethodContent {
  fn from(content: WasmMethodContent) -> Self {
    match content.0 {
      WasmMethodContentInner::Generate => MethodContent::Generate,
      WasmMethodContentInner::Private(private_key) => MethodContent::Private(PrivateKey::from(private_key)),
      WasmMethodContentInner::Public(public_key) => MethodContent::Public(PublicKey::from(public_key)),
    }
  }
}

impl From<MethodContent> for WasmMethodContent {
  fn from(content: MethodContent) -> Self {
    WasmMethodContent(match content {
      MethodContent::Generate => WasmMethodContentInner::Generate,
      MethodContent::Private(private_key) => WasmMethodContentInner::Private(private_key.as_ref().to_vec()),
      MethodContent::Public(public_key) => WasmMethodContentInner::Public(public_key.as_ref().to_vec()),
    })
  }
}
