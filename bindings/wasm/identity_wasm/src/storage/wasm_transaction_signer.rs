// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_iota::iota::rebased::client::IotaKeySignature;
use identity_iota::iota_interaction::types::crypto::PublicKey;
use identity_iota::iota_interaction::types::crypto::Signature;
use iota_interaction_ts::WasmIotaSignature;
use iota_interaction_ts::WasmPublicKey;
use secret_storage::Error as SecretStorageError;
use secret_storage::Signer;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::error::Result;

#[wasm_bindgen(typescript_custom_section)]
const I_TX_SIGNER: &str = r#"
import { PublicKey } from "@iota/iota-sdk/cryptography";
import { Signature } from "@iota/iota-sdk/client";

interface TransactionSigner {
  sign: (data: Uint8Array) => Promise<Signature>;
  publicKey: () => Promise<PublicKey>;
  keyId: () => string;
}
"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "TransactionSigner")]
  pub type WasmTransactionSigner;

  #[wasm_bindgen(method, catch)]
  pub async fn sign(this: &WasmTransactionSigner, data: &[u8]) -> Result<WasmIotaSignature>;
  #[wasm_bindgen(js_name = "publicKey", method, catch)]
  pub async fn public_key(this: &WasmTransactionSigner) -> Result<WasmPublicKey>;

  // TODO: re-add WasmTransactionSigner::key_id
  // #[wasm_bindgen(js_name = "keyId", structural, method)]
  // // pub fn key_id(this: &WasmTransactionSigner) -> js_sys::JsString;
  // pub fn key_id(this: &WasmTransactionSigner) -> String;
}

#[async_trait(?Send)]
impl Signer<IotaKeySignature> for WasmTransactionSigner {
  type KeyId = String;

  async fn sign(&self, data: &Vec<u8>) -> std::result::Result<Signature, SecretStorageError> {
    self.sign(data).await.and_then(|v| v.try_into()).map_err(|err| {
      let details = err.as_string().map(|v| format!("; {}", v)).unwrap_or_default();
      let message = format!("could not sign data{details}");
      SecretStorageError::Other(anyhow::anyhow!(message))
    })
  }

  async fn public_key(&self) -> std::result::Result<PublicKey, SecretStorageError> {
    self.public_key().await.and_then(|v| v.try_into()).map_err(|err| {
      let details = err.as_string().map(|v| format!("; {}", v)).unwrap_or_default();
      let message = format!("could not get public key{details}");
      SecretStorageError::KeyNotFound(message)
    })
  }

  fn key_id(&self) -> &String {
    todo!("WasmTransactionSigner::key_id");
  }
}
