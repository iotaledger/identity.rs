use identity_iota::storage::key_storage::JwkGenOutput;
use identity_iota::storage::key_storage::KeyId;
use wasm_bindgen::prelude::*;

use super::WasmJwk;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[wasm_bindgen(js_name = JwkGenOutput, inspectable)]
pub struct WasmJwkGenOutput(pub(crate) JwkGenOutput);

#[wasm_bindgen(js_class = JwkGenOutput)]
impl WasmJwkGenOutput {
  #[wasm_bindgen(constructor)]
  pub fn new(key_id: String, jwk: WasmJwk) -> Self {
    Self(JwkGenOutput {
      key_id: KeyId::new(key_id),
      jwk: jwk.into(),
    })
  }
}

impl From<WasmJwkGenOutput> for JwkGenOutput {
  fn from(value: WasmJwkGenOutput) -> Self {
    value.0
  }
}

impl From<JwkGenOutput> for WasmJwkGenOutput {
  fn from(value: JwkGenOutput) -> Self {
    WasmJwkGenOutput(value)
  }
}

impl_wasm_json!(WasmJwkGenOutput, JwkGenOutput);
impl_wasm_clone!(WasmJwkGenOutput, JwkGenOutput);
