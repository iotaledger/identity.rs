// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::storage::KeyId;
use identity_iota::storage::StorageSigner;
use identity_iota::verification::jwk::JwkParams;
use identity_iota::verification::jwu;
use iota_interaction_ts::WasmIotaSignature;
use iota_interaction_ts::WasmPublicKey;
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
  pub async fn sign(&self, data: Vec<u8>) -> Result<WasmIotaSignature> {
    let sig = self.signer().sign(&data).await.wasm_result()?;
    sig.try_into()
  }

  #[wasm_bindgen(js_name = publicKey)]
  pub async fn public_key(&self) -> Result<WasmPublicKey> {
    Signer::public_key(&self.signer())
      .await
      .wasm_result()
      .and_then(|pk| WasmPublicKey::try_from(&pk))
  }
}
