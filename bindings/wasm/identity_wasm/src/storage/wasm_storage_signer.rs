// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::iota_interaction::IotaKeySignature;
use identity_iota::storage::KeyId;
use identity_iota::verification::jwk::JwkParams;
use identity_iota::verification::jwu;
use secret_storage::SignatureScheme;
use secret_storage::Signer;
use wasm_bindgen::prelude::*;

use crate::error::wasm_error;
use crate::error::Result;
use crate::jose::WasmJwk;
use crate::storage::WasmStorage;

use super::StorageSignerOwned;

#[wasm_bindgen(js_name = StorageSigner)]
pub struct WasmStorageSigner(pub(crate) StorageSignerOwned);

#[wasm_bindgen(js_class = StorageSigner)]
impl WasmStorageSigner {
  #[wasm_bindgen(constructor)]
  pub fn new(storage: WasmStorage, key_id: String, public_key: WasmJwk) -> Self {
    let signer = StorageSignerOwned::new(
      storage,
      KeyId::new(&key_id),
      public_key.0,
    );
    Self(signer)
  }

  #[wasm_bindgen(js_name = keyId)]
  pub fn key_id(&self) -> String {
    self.0.key_id().to_string()
  }

  #[wasm_bindgen(js_name = sign)]
  pub async fn sign(
    &self,
    data: &[u8],
  ) -> Result<<IotaKeySignature as SignatureScheme>::Signature> {
    self.0.sign(data).await.map_err(wasm_error)
  }

  #[wasm_bindgen(js_name = publicKeyRaw)]
  pub fn public_key_raw(&self) -> Result<<IotaKeySignature as SignatureScheme>::PublicKey> {
    let jwk = self.0.public_key().clone();

    match jwk.params() {
      JwkParams::Okp(params) => jwu::decode_b64(&params.x)
        .map_err(|e| JsValue::from_str(&format!("could not base64 decode key {}; {e}", self.key_id()))),
      _ => todo!("add support for other key types"),
    }
  }
}
