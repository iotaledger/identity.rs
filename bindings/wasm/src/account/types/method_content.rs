// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::MethodContent;
use identity::crypto::{PrivateKey, PublicKey};
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "MethodContent | undefined")]
  pub type OptionMethodContent;
}

// Workaround for having to deserialize `WasmMethodContent` from a Typescript interface
// TODO: remove when https://github.com/rustwasm/wasm-bindgen/pull/2677 is merged.
#[derive(Serialize, Deserialize)]
enum WasmMethodContentInner {
  GenerateEd25519,
  PrivateEd25519(Vec<u8>),
  PublicEd25519(Vec<u8>),
  GenerateX25519,
  PrivateX25519(Vec<u8>),
  PublicX25519(Vec<u8>),
}

#[wasm_bindgen(js_name = MethodContent, inspectable)]
#[derive(Serialize, Deserialize)]
pub struct WasmMethodContent(WasmMethodContentInner);

#[wasm_bindgen(js_class = MethodContent)]
impl WasmMethodContent {
  /// Generate and store a new Ed25519 keypair for a new `Ed25519VerificationKey2018` method.
  #[wasm_bindgen(js_name = GenerateEd25519)]
  pub fn generate_ed25519() -> WasmMethodContent {
    Self(WasmMethodContentInner::GenerateEd25519)
  }

  /// Store an existing Ed25519 private key and derive a public key from it for a new
  /// `Ed25519VerificationKey2018` method.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = PrivateEd25519)]
  pub fn private_ed25519(privateKey: Vec<u8>) -> WasmMethodContent {
    Self(WasmMethodContentInner::PrivateEd25519(privateKey))
  }

  /// Insert an existing Ed25519 public key into a new `Ed25519VerificationKey2018` method,
  /// without generating or storing a private key.
  ///
  /// NOTE: the method will be unable to be used to sign anything without a private key.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = PublicEd25519)]
  pub fn public_ed25519(publicKey: Vec<u8>) -> WasmMethodContent {
    Self(WasmMethodContentInner::PublicEd25519(publicKey))
  }

  /// Generate and store a new X25519 keypair for a new `X25519KeyAgreementKey2019` method.
  #[wasm_bindgen(js_name = GenerateX25519)]
  pub fn generate_x25519() -> WasmMethodContent {
    Self(WasmMethodContentInner::GenerateX25519)
  }

  /// Store an existing X25519 private key and derive a public key from it for a new
  /// `X25519KeyAgreementKey2019` method.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = PrivateX25519)]
  pub fn private_x25519(privateKey: Vec<u8>) -> WasmMethodContent {
    Self(WasmMethodContentInner::PrivateX25519(privateKey))
  }

  /// Insert an existing X25519 public key into a new `X25519KeyAgreementKey2019` method,
  /// without generating or storing a private key.
  ///
  /// NOTE: the method will be unable to be used for key exchange without a private key.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = PublicX25519)]
  pub fn public_x25519(publicKey: Vec<u8>) -> WasmMethodContent {
    Self(WasmMethodContentInner::PublicX25519(publicKey))
  }

  /// Serializes `MethodContent` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes `MethodContent` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json_value: JsValue) -> Result<WasmMethodContent> {
    json_value.into_serde::<WasmMethodContentInner>().map(Self).wasm_result()
  }
}

impl From<WasmMethodContent> for MethodContent {
  fn from(content: WasmMethodContent) -> Self {
    match content.0 {
      WasmMethodContentInner::GenerateEd25519 => MethodContent::GenerateEd25519,
      WasmMethodContentInner::PrivateEd25519(private_key) => MethodContent::PrivateEd25519(PrivateKey::from(private_key)),
      WasmMethodContentInner::PublicEd25519(public_key) => MethodContent::PublicEd25519(PublicKey::from(public_key)),
      WasmMethodContentInner::GenerateX25519 => MethodContent::GenerateX25519,
      WasmMethodContentInner::PrivateX25519(private_key) => MethodContent::PrivateX25519(PrivateKey::from(private_key)),
      WasmMethodContentInner::PublicX25519(public_key) => MethodContent::PublicX25519(PublicKey::from(public_key)),
    }
  }
}
