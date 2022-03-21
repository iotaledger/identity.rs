// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::decode_b58;
use identity::crypto::KeyExchange;
use identity::crypto::X25519;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// An implementation of `X25519` Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange.
#[wasm_bindgen(js_name = X25519)]
pub struct WasmX25519(pub(crate) X25519);

#[wasm_bindgen(js_class = X25519)]
impl WasmX25519 {
  /// Performs a cryptographic key exchange process (e.g. Diffie-Hellman) using the private key
  /// of the first party with with the public key of the second party, resulting in a shared secret.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = keyExchange)]
  // TODO: refactor private key type to UInt8Array
  // Named privateKey to avoid reserved keyword private in Javascript.
  pub fn key_exchange(privateKey: String, publicKey: Vec<u8>) -> Result<Vec<u8>> {
    let private: Vec<u8> = decode_b58(&privateKey).wasm_result()?;
    X25519::key_exchange(&private, &publicKey)
      .map(|bytes| bytes.to_vec())
      .wasm_result()
  }
}
