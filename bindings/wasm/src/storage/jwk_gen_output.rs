use identity_iota::storage::key_storage::JwkGenOutput;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[wasm_bindgen(js_name = JwkGenOutput, inspectable)]
pub struct WasmJwkGenOutput(pub(crate) JwkGenOutput);

impl WasmJwkGenOutput {
  #[allow(non_snake_case)]
  pub fn new(jwkGenOutput: JwkGenOutput) -> Self {
    Self(jwkGenOutput)
  }
}

impl From<WasmJwkGenOutput> for JwkGenOutput {
  fn from(value: WasmJwkGenOutput) -> Self {
    value.0
  }
}

impl From<JwkGenOutput> for WasmJwkGenOutput {
  fn from(value: JwkGenOutput) -> Self {
    WasmJwkGenOutput::new(value)
  }
}

impl_wasm_json!(WasmJwkGenOutput, JwkGenOutput);
impl_wasm_clone!(WasmJwkGenOutput, JwkGenOutput);
