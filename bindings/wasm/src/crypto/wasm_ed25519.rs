// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::crypto::Ed25519;
use identity_iota::crypto::PrivateKey;
use identity_iota::crypto::PublicKey;
use identity_iota::crypto::Sign;
use identity_iota::crypto::Verify;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = Ed25519, inspectable)]
pub struct WasmEd25519(pub(crate) Ed25519);

#[wasm_bindgen(js_class = Ed25519)]
impl WasmEd25519 {
  /// Length in bytes of an Ed25519 private key.
  #[wasm_bindgen(js_name = PRIVATE_KEY_LENGTH)]
  pub fn private_key_length() -> usize {
    Ed25519::PRIVATE_KEY_LENGTH
  }

  /// Length in bytes of an Ed25519 public key.
  #[wasm_bindgen(js_name = PUBLIC_KEY_LENGTH)]
  pub fn public_key_length() -> usize {
    Ed25519::PUBLIC_KEY_LENGTH
  }

  /// Length in bytes of an Ed25519 signature.
  #[wasm_bindgen(js_name = SIGNATURE_LENGTH)]
  pub fn signature_length() -> usize {
    Ed25519::SIGNATURE_LENGTH
  }

  /// Computes an EdDSA signature using an Ed25519 private key.
  ///
  /// NOTE: this differs from [Document.signData](#Document+signData) which uses JCS
  /// to canonicalize JSON messages.
  ///
  /// The private key must be a 32-byte seed in compliance with [RFC 8032](https://datatracker.ietf.org/doc/html/rfc8032#section-3.2).
  /// Other implementations often use another format. See [this blog post](https://blog.mozilla.org/warner/2011/11/29/ed25519-keys/) for further explanation.
  #[allow(non_snake_case)]
  #[wasm_bindgen]
  pub fn sign(message: &[u8], privateKey: Vec<u8>) -> Result<Vec<u8>> {
    let key: PrivateKey = privateKey.into();
    Ed25519::sign(message, &key)
      .map(|signature| signature.to_vec())
      .wasm_result()
  }

  /// Verifies an EdDSA signature against an Ed25519 public key.
  ///
  /// NOTE: this differs from {@link #Document+verifyData} which uses JCS
  /// to canonicalize JSON messages.
  #[allow(non_snake_case)]
  #[wasm_bindgen]
  pub fn verify(message: &[u8], signature: &[u8], publicKey: Vec<u8>) -> Result<()> {
    let key: PublicKey = publicKey.into();
    Ed25519::verify(message, signature, &key).wasm_result()
  }
}
