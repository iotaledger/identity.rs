// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::account::{IdentitySetup, MethodSecret};
use identity::core::decode_b58;
use identity::crypto::{KeyType, PrivateKey};
use crate::crypto::{KeyCollection, KeyType as WasmKeyType};
use wasm_bindgen::prelude::*;
use crate::error::wasm_error;
use crate::error::{Result, WasmResult};

#[wasm_bindgen(js_name = MethodSecret)]
pub struct WasmMethodSecret(pub(crate) MethodSecret);

#[wasm_bindgen(js_class = MethodSecret)]
impl WasmMethodSecret {
  /// Creates a {@link MethodSecret} object from base58-encoded Ed25519 private key.
  #[wasm_bindgen(js_name = ed25519_base58)]
  pub fn Ed25519_base58(private_key: &str) -> Result<WasmMethodSecret> {
    let private: PrivateKey = decode_b58(private_key).map_err(wasm_error)?.into();
    Ok(Self(MethodSecret::Ed25519(private)))
  }

  /// Creates a {@link MethodSecret} object from {@link KeyCollection}.
  #[wasm_bindgen(js_name = merkelKeyCollection)]
  pub fn merkel_key_collection(collection: KeyCollection) -> WasmMethodSecret{
    Self(MethodSecret::MerkleKeyCollection(collection.0))
  }
}