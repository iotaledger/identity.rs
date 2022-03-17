// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::decode_b58;
use identity::crypto::Ed25519;
use identity::crypto::PrivateKey;
use identity::crypto::PublicKey;
use identity::crypto::Sign;
use identity::crypto::Verify;
use wasm_bindgen::prelude::*;

use crate::error::wasm_error;
use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = Ed25519, inspectable)]
pub struct WasmEd25519(pub(crate) Ed25519);

#[wasm_bindgen(js_class = Ed25519)]
impl WasmEd25519 {
  /// Signs the given `message` with a base58 encoded `key`.
  #[wasm_bindgen]
  pub fn sign(message: &[u8], key: &str) -> Result<Vec<u8>> {
    let key: PrivateKey = decode_b58(key).map_err(wasm_error)?.into();
    Ed25519::sign(message, &key)
      .map(|signature| signature.to_vec())
      .wasm_result()
  }

  #[wasm_bindgen]
  pub fn verify(message: &[u8], signature: &[u8], key: &str) -> Result<()> {
    let key: PublicKey = decode_b58(key).map_err(wasm_error)?.into();
    Ed25519::verify(message, signature, &key).wasm_result()
  }
}
