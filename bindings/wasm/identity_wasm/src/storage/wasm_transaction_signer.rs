// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use identity_iota::iota::rebased::client::IotaKeySignature;
use identity_iota::iota_interaction::types::crypto::PublicKey;
use identity_iota::iota_interaction::types::crypto::Signature;
use identity_iota::iota_interaction::types::crypto::SignatureScheme;
use iota_interaction_ts::WasmIotaSignature;
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
  iotaPublicKeyBytes: () => Promise<Uint8Array>;
  keyId: () => string;
}
"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "TransactionSigner")]
  pub type WasmTransactionSigner;

  #[wasm_bindgen(method, structural, catch)]
  pub async fn sign(this: &WasmTransactionSigner, data: &[u8]) -> Result<WasmIotaSignature>;

  #[wasm_bindgen(js_name = "iotaPublicKeyBytes", method, structural, catch)]
  pub async fn iota_public_key_bytes(this: &WasmTransactionSigner) -> Result<js_sys::Uint8Array>;

  #[wasm_bindgen(js_name = "keyId", method, structural)]
  pub fn key_id(this: &WasmTransactionSigner) -> String;
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
    let uint8_array = self.iota_public_key_bytes().await.map_err(|err| {
      let details = err.as_string().map(|v| format!("; {}", v)).unwrap_or_default();
      let message = format!("could not get public key{details}");
      SecretStorageError::KeyNotFound(message)
    })?;

    let raw_bytes = uint8_array.to_vec();
    let signature_scheme = SignatureScheme::from_flag_byte(&raw_bytes[0]).map_err(|err| {
      let details = format!("; {}", err);
      let message = format!("could parse scheme flag of public key, {details}");
      SecretStorageError::Other(anyhow::anyhow!(message))
    })?;

    PublicKey::try_from_bytes(signature_scheme, &raw_bytes[1..]).map_err(|err| {
      let details = format!("; {}", err);
      let message = format!("could parse public key from bytes, {details}");
      SecretStorageError::Other(anyhow::anyhow!(message))
    })
  }

  fn key_id(&self) -> String {
    self.key_id()
  }
}
