// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use wasm_bindgen::prelude::wasm_bindgen;
use identity_iota::iota::rebased::client::IotaKeySignature;
use secret_storage::Error as SecretStorageError;
use secret_storage::Signer;

use crate::error::Result;

#[wasm_bindgen(typescript_custom_section)]
const I_TX_SIGNER: &str = r#"
interface TransactionSigner {
  sign: (data: Uint8Array) => Promise<Uint8Array>;
  publicKey: () => Promise<Uint8Array>;
  keyId: () => string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "TransactionSigner")]
    pub type WasmTransactionSigner;

    #[wasm_bindgen(js_name = "sign", structural, method, catch)]
    pub async fn sign(this: &WasmTransactionSigner, data: &[u8]) -> Result<js_sys::Uint8Array>;

    #[wasm_bindgen(js_name = "publicKey", structural, method, catch)]
    pub async fn public_key(this: &WasmTransactionSigner) -> Result<js_sys::Uint8Array>;

    // TODO: re-add WasmTransactionSigner::key_id
    // #[wasm_bindgen(js_name = "keyId", structural, method)]
    // // pub fn key_id(this: &WasmTransactionSigner) -> js_sys::JsString;
    // pub fn key_id(this: &WasmTransactionSigner) -> String;
}

#[async_trait(?Send)]
impl Signer<IotaKeySignature> for WasmTransactionSigner {
    type KeyId = String;

    async fn sign(&self, data: &[u8]) -> std::result::Result<Vec<u8>, SecretStorageError> {
        self.sign(data).await.map(|v| v.to_vec()).map_err(|err| {
            let details = err.as_string()
                .map(|v| format!("; {}", v))
                .unwrap_or_default();
            let message = format!("could not sign data{details}");
            SecretStorageError::Other(anyhow::anyhow!(message))
        })
    }

    async fn public_key(&self) -> std::result::Result<Vec<u8>, SecretStorageError> {
        self.public_key().await.map(|v| v.to_vec()).map_err(|err| {
            let details = err.as_string()
                .map(|v| format!("; {}", v))
                .unwrap_or_default();
            let message = format!("could not get public key{details}");
            SecretStorageError::KeyNotFound(message)
        })
    }
    
    fn key_id(&self) -> &String {
        todo!("WasmTransactionSigner::key_id");
    }
}