// Copyright 2020-2025 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crate::error::Result;
use crate::error::WasmResult;
use identity_iota_interaction::types::base_types::IotaAddress;
use identity_iota_interaction::KeytoolSigner;
use identity_iota_interaction::KeytoolSignerBuilder;
use secret_storage::Signer;
use serde_json::Value;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsError;

use crate::bindings::WasmIotaSignature;
use crate::bindings::WasmPublicKey;

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

/// An implementation of the Signer interface that relies on
/// IOTA Keytool.
#[wasm_bindgen(js_name = KeytoolSigner)]
pub struct WasmKeytoolSigner(pub(crate) KeytoolSigner);

#[wasm_bindgen(js_class = KeytoolSigner)]
impl WasmKeytoolSigner {
  /// Returns a new {@link KeytoolSigner}. The optional parameters respectively
  /// allow to set the signing address and the `iota` binary to use. Defaults
  /// to use the current active address and the binary found in $PATH.
  #[wasm_bindgen(constructor)]
  pub fn new(address: Option<String>, iota_bin_location: Option<String>) -> Result<WasmKeytoolSigner> {
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

    Ok(WasmKeytoolSigner(builder.build().wasm_result()?))
  }

  /// Returns the signing address.
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

  #[wasm_bindgen(js_name = iotaPublicKeyBytes)]
  pub async fn iota_public_key_bytes(&self) -> Vec<u8> {
    let pk = self.0.public_key();
    let mut bytes = vec![pk.flag()];
    bytes.extend_from_slice(pk.as_ref());

    bytes
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
