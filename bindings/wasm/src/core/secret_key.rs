// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519;
use identity::account_storage::Error as AccountStorageError;
use identity::core::decode_b58;
use identity::core::encode_b58;
use identity::crypto::PrivateKey;
use identity::crypto::PublicKey;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = Ed25519PrivateKey)]
pub struct WasmEd25519PrivateKey(pub(crate) ed25519::SecretKey);

#[wasm_bindgen(js_class = Ed25519PrivateKey)]
impl WasmEd25519PrivateKey {
  /// Create a new `Ed25519PrivateKey` from a base58 encoded string.
  #[wasm_bindgen(js_name = "fromBase58")]
  pub fn from_base58(private_key: &str) -> Result<WasmEd25519PrivateKey> {
    let private_key: PrivateKey = decode_b58(private_key).wasm_result()?.into();
    let private_key_bytes: [u8; 32] = <[u8; 32]>::try_from(private_key.as_ref())
      .map_err(|err| AccountStorageError::InvalidPrivateKey(format!("expected a slice of 32 bytes - {}", err)))
      .wasm_result()?;
    Ok(WasmEd25519PrivateKey(ed25519::SecretKey::from_bytes(private_key_bytes)))
  }

  /// Returns a base58 encoded string that represents the PublicKey.
  #[wasm_bindgen(js_name = publicKey)]
  pub fn public_key(&self) -> String {
    let public: ed25519::PublicKey = self.0.public_key();
    let public_key: PublicKey = public.to_bytes().to_vec().into();
    encode_b58(&public_key)
  }
}
