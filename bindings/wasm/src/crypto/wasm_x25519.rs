// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::core::decode_b58;
use identity::crypto::X25519;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// An implementation of `X25519` Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange.
#[wasm_bindgen(js_name = X25519)]
pub struct WasmX25519;

#[wasm_bindgen(js_class = X25519)]
impl WasmX25519 {
  /// Performs a cryptographic key exchange process (e.g. Diffie-Hellman) using the private key
  /// of the first party with the public key of the second party, resulting in a shared secret.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = keyExchange)]
  // TODO: refactor private key type to UInt8Array
  pub fn key_exchange(privateKey: String, publicKey: Vec<u8>) -> Result<Vec<u8>> {
    let private: Vec<u8> = decode_b58(&privateKey).wasm_result()?;
    X25519::key_exchange(&private, &publicKey)
      .map(|bytes| bytes.to_vec())
      .wasm_result()
  }

  /// Transforms an `Ed25519` private key to an `X25519` private key.
  ///
  /// This is possible because Ed25519 is birationally equivalent to Curve25519 used by X25519.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = Ed25519toX25519Private)]
  pub fn ed25519_to_x25519_private(privateKey: &[u8]) -> Result<Vec<u8>> {
    X25519::ed25519_to_x25519_private(privateKey)
      .map(|key| key.as_ref().to_vec())
      .wasm_result()
  }

  /// Transforms an `Ed25519` public key to an `X25519` public key.
  ///
  /// This is possible because Ed25519 is birationally equivalent to Curve25519 used by X25519.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = Ed25519toX25519Public)]
  pub fn ed25519_to_x25519_public(publicKey: &[u8]) -> Result<Vec<u8>> {
    X25519::ed25519_to_x25519_public(publicKey)
      .map(|key| key.as_ref().to_vec())
      .wasm_result()
  }
}
