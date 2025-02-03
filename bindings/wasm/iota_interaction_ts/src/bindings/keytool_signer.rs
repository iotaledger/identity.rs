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
use wasm_bindgen::JsValue;

#[wasm_bindgen(typescript_custom_section)]
const NODE_EXEC_HANDLER: &str = r#"
const utils = require("util");
const exec = utils.promisify(require("child_process").exec);

async function exec_handler(cmd: string): Promise<unknown> {
  try {
    const { stdout } = await exec(cmd);
    return JSON.parse(stdout) as unknown;
  } catch(e) {
    console.error(e);
  }
}
"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(catch)]
  async fn exec_handler(cmd: &str) -> Result<JsValue>;
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
}

#[async_trait(?Send)]
impl Signer<IotaKeySignature> for WasmKeytoolSigner {
  type KeyId = IotaAddress;

  fn key_id(&self) -> &Self::KeyId {
    self.0.key_id()
  }

  async fn public_key(&self) -> std::result::Result<PublicKey, SecretStorageError> {
    Ok(self.0.public_key().clone())
  }

  async fn sign(&self, data: &TransactionDataBcs) -> std::result::Result<Signature, SecretStorageError> {
    self.0.sign(data).await
  }
}

// This is used in KeytoolSigner implementation to issue CLI commands.
#[no_mangle]
fn __wasm_exec_iota_cmd(cmd: &str) -> anyhow::Result<Value> {
  let output = futures::executor::block_on(exec_handler(cmd)).map_err(|_| anyhow::anyhow!("exec failed"))?;
  serde_wasm_bindgen::from_value(output)
    .map_err(|_| anyhow::anyhow!("failed to deserialize JSON object from command output"))
}
