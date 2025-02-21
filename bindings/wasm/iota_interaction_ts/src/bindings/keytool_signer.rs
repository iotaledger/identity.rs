use std::path::PathBuf;
use std::str::FromStr;

use crate::error::Result;
use crate::error::WasmResult;
use async_trait::async_trait;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::types::crypto::PublicKey;
use identity_iota_interaction::types::crypto::Signature;
use identity_iota_interaction::IotaKeySignature;
use identity_iota_interaction::KeytoolSigner;
use identity_iota_interaction::KeytoolSignerBuilder;
use identity_iota_interaction::TransactionDataBcs;
use secret_storage::Error as SecretStorageError;
use secret_storage::Signer;
use serde_json::Value;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsError;
use wasm_bindgen::JsValue;

use super::WasmIotaSignature;
use super::WasmPublicKey;

#[wasm_bindgen(module = buffer)]
extern "C" {
  #[wasm_bindgen(typescript_type = Buffer)]
  type NodeBuffer;
  #[wasm_bindgen(method, js_name = toString)]
  fn to_string(this: &NodeBuffer) -> String;
}

#[wasm_bindgen(module = child_process)]
extern "C" {
  #[wasm_bindgen(js_name = execSync, catch)]
  fn exec_cli_cmd(cmd: &str) -> Result<NodeBuffer>;
}

#[wasm_bindgen(js_name = KeytoolSigner)]
pub struct WasmKeytoolSigner(pub(crate) KeytoolSigner);

#[wasm_bindgen(js_class = KeytoolSigner)]
impl WasmKeytoolSigner {
  #[wasm_bindgen(constructor)]
  pub async fn new(address: Option<String>, iota_bin_location: Option<String>) -> Result<WasmKeytoolSigner> {
    let address = address
      .as_deref()
      .map(IotaAddress::from_str)
      .transpose()
      .wasm_result()?;

    let builder = address
      .map(|address| KeytoolSignerBuilder::new().with_address(address))
      .unwrap_or_default();
    let builder = if let Some(iota_bin_location) = iota_bin_location {
      builder.iota_bin_location(iota_bin_location)
    } else {
      builder
    };

    Ok(WasmKeytoolSigner(builder.build().await.wasm_result()?))
  }

  #[wasm_bindgen]
  pub fn address(&self) -> String {
    self.0.address().to_string()
  }

  // These method definition are needed to make sure `KeytoolSigner`
  // implements `Signer` interface.

  #[wasm_bindgen(js_name = keyId)]
  pub fn key_id(&self) -> String {
    self.address()
  }

  #[wasm_bindgen(js_name = publicKey)]
  pub async fn public_key(&self) -> Result<WasmPublicKey> {
    self.0.public_key().try_into()
  }

  #[wasm_bindgen]
  pub async fn sign(&self, data: Vec<u8>) -> Result<WasmIotaSignature> {
    self
      .0
      .sign(&data)
      .await
      .map_err(|e| JsError::new(&e.to_string()).into())
      .and_then(|sig| sig.try_into())
  }
}

// This is used in KeytoolSigner implementation to issue CLI commands.
#[no_mangle]
pub extern "Rust" fn __wasm_exec_iota_cmd(cmd: &str) -> anyhow::Result<Value> {
  let output = exec_cli_cmd(cmd)
    .map_err(|e| anyhow::anyhow!("exec failed: {e:?}"))?
    .to_string();
  serde_json::from_str(&output).map_err(|_| anyhow::anyhow!("failed to deserialize JSON object from command output"))
}
