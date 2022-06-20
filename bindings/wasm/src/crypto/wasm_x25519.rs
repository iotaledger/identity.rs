// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::crypto::X25519;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// An implementation of `X25519` Elliptic-curve Diffie-Hellman (ECDH) cryptographic key exchange.
#[wasm_bindgen(js_name = X25519)]
pub struct WasmX25519;

#[wasm_bindgen(js_class = X25519)]
impl WasmX25519 {
  /// Length in bytes of an X25519 private key.
  #[wasm_bindgen(js_name = PRIVATE_KEY_LENGTH)]
  pub fn private_key_length() -> usize {
    X25519::PRIVATE_KEY_LENGTH
  }

  /// Length in bytes of an X25519 public key.
  #[wasm_bindgen(js_name = PUBLIC_KEY_LENGTH)]
  pub fn public_key_length() -> usize {
    X25519::PUBLIC_KEY_LENGTH
  }

  /// Performs Diffie-Hellman key exchange using the private key of the first party with the
  /// public key of the second party, resulting in a shared secret.
  #[allow(non_snake_case)]
  #[wasm_bindgen(js_name = keyExchange)]
  pub fn key_exchange(privateKey: Vec<u8>, publicKey: Vec<u8>) -> Result<Vec<u8>> {
    X25519::key_exchange(&privateKey, &publicKey)
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
