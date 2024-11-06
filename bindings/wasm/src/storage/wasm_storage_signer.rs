// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Borrow;
use std::rc::Rc;

use identity_iota::iota::iota_sdk_abstraction::IotaKeySignature;
use identity_iota::storage::KeyId;
use identity_iota::storage::Storage;
use identity_iota::storage::StorageSigner;
use secret_storage::SignatureScheme;
use secret_storage::Signer;
use wasm_bindgen::prelude::*;

use crate::error::wasm_error;
use crate::error::Result;
use crate::jose::WasmJwk;
use crate::storage::WasmJwkStorage;
use crate::storage::WasmKeyIdStorage;
use crate::storage::WasmStorage;

#[wasm_bindgen(js_name = StorageSigner)]
pub struct WasmStorageSigner(pub(crate) StorageSigner<WasmJwkStorage, WasmKeyIdStorage>);

#[wasm_bindgen(js_class = StorageSigner)]
impl WasmStorageSigner {
    #[wasm_bindgen(constructor)]
    pub fn new(storage: &WasmStorage, key_id: String, public_key: WasmJwk) -> Self {
        let signer = StorageSigner::new_with_shared_storage(
            storage.0.clone(),
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
}
