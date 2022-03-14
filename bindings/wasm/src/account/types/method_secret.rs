// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::MethodSecret;
use identity::core::decode_b58;
use identity::crypto::PrivateKey;
use wasm_bindgen::prelude::*;

use crate::crypto::WasmKeyCollection;
use crate::error::wasm_error;
use crate::error::Result;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "MethodSecret | undefined")]
  pub type OptionMethodSecret;
}

#[wasm_bindgen(js_name = MethodSecret)]
pub struct WasmMethodSecret(pub(crate) MethodSecret);

#[wasm_bindgen(js_class = MethodSecret)]
impl WasmMethodSecret {
  /// Creates a {@link MethodSecret} object from base58-encoded Ed25519 private key.
  #[wasm_bindgen(js_name = ed25519Base58)]
  pub fn ed25519_base58(private_key: &str) -> Result<WasmMethodSecret> {
    let private: PrivateKey = decode_b58(private_key).map_err(wasm_error)?.into();
    Ok(Self(MethodSecret::Ed25519(private)))
  }

  /// Creates a {@link MethodSecret} object from {@link KeyCollection}.
  #[wasm_bindgen(js_name = merkleKeyCollection)]
  pub fn merkle_key_collection(collection: &WasmKeyCollection) -> WasmMethodSecret {
    Self(MethodSecret::MerkleKeyCollection(collection.0.clone()))
  }
}
