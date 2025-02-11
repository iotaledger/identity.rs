// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::storage::KeyId;
use identity_iota::storage::StorageSigner;
use identity_iota::verification::jwk::JwkParams;
use identity_iota::verification::jwu;
use secret_storage::Signer;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;
use crate::jose::WasmJwk;
use crate::storage::WasmJwkStorage;
use crate::storage::WasmKeyIdStorage;
use crate::storage::WasmStorage;

#[wasm_bindgen(js_name = StorageSigner)]
pub struct WasmStorageSigner {
  storage: WasmStorage,
  key_id: KeyId,
  public_key: WasmJwk,
}

impl WasmStorageSigner {
  fn signer(&self) -> StorageSigner<'_, WasmJwkStorage, WasmKeyIdStorage> {
    StorageSigner::new(&self.storage.0, self.key_id.clone(), self.public_key.0.clone())
  }
}

#[wasm_bindgen(js_class = StorageSigner)]
impl WasmStorageSigner {
  #[wasm_bindgen(constructor)]
  pub fn new(storage: &WasmStorage, key_id: String, public_key: WasmJwk) -> Self {
    Self {
      storage: storage.clone(),
      key_id: KeyId::new(key_id),
      public_key,
    }
  }

  #[wasm_bindgen(js_name = keyId)]
  pub fn key_id(&self) -> String {
    self.key_id.to_string()
  }

  #[wasm_bindgen(js_name = sign)]
  pub async fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
    self.signer().sign(data).await.wasm_result()
  }

  #[wasm_bindgen(js_name = publicKey)]
  pub async fn public_key(&self) -> Result<Vec<u8>> {
    let jwk = &self.public_key.0;

    match jwk.params() {
      JwkParams::Okp(params) => jwu::decode_b64(&params.x)
        .map_err(|e| JsValue::from_str(&format!("could not base64 decode key {}; {e}", &self.key_id))),
      _ => todo!("add support for other key types"),
    }
  }
}
