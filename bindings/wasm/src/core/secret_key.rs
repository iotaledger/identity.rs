// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519;
use identity::account_storage::Error as AccountStorageError;
use identity::crypto::PrivateKey;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

#[wasm_bindgen(js_name = Ed25519PrivateKey)]
pub struct WasmEd25519PrivateKey(pub(crate) ed25519::SecretKey);

#[wasm_bindgen(js_class = Ed25519PrivateKey)]
impl WasmEd25519PrivateKey {
  /// Create a new `Ed25519PrivateKey` from a 'Uint8Array'.
  #[wasm_bindgen(js_name = "fromBase58")]
  pub fn from_key(private_key: Vec<u8>) -> Result<WasmEd25519PrivateKey> {
    let private_key: PrivateKey = private_key.into();
    let private_key_bytes: [u8; 32] = <[u8; 32]>::try_from(private_key.as_ref())
      .map_err(|err| AccountStorageError::InvalidPrivateKey(format!("expected a slice of 32 bytes - {}", err)))
      .wasm_result()?;
    Ok(WasmEd25519PrivateKey(ed25519::SecretKey::from_bytes(private_key_bytes)))
  }

  /// Returns the PublicKey as a `Uint8Array`.
  #[wasm_bindgen(js_name = publicKey)]
  pub fn public_key(&self) -> Vec<u8> {
    let public: ed25519::PublicKey = self.0.public_key();
    public.to_bytes().to_vec()
  }
}
