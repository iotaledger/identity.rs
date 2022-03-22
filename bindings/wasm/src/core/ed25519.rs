// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::crypto::Ed25519;
use identity::crypto::PrivateKey;
use identity::crypto::PublicKey;
use identity::crypto::Sign;
use identity::crypto::Verify;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = Ed25519, inspectable)]
pub struct WasmEd25519(pub(crate) Ed25519);

#[wasm_bindgen(js_class = Ed25519)]
impl WasmEd25519 {
  /// Signs the `message` with the given `Key`.
  #[wasm_bindgen]
  pub fn sign(message: &[u8], key: Vec<u8>) -> Result<Vec<u8>> {
    let key: PrivateKey = key.into();
    Ed25519::sign(message, &key)
      .map(|signature| signature.to_vec())
      .wasm_result()
  }

  #[wasm_bindgen]
  pub fn verify(message: &[u8], signature: &[u8], key: Vec<u8>) -> Result<()> {
    let key: PublicKey = key.into();
    Ed25519::verify(message, signature, &key).wasm_result()
  }
}
