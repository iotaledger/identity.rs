use identity_jose::jwk::Jwk;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[wasm_bindgen(js_name = Jwk, inspectable)]
pub struct WasmJwk(pub(crate) Jwk);

impl WasmJwk {
  pub fn new(jwk: Jwk) -> Self {
    Self(jwk)
  }
}

impl From<WasmJwk> for Jwk {
  fn from(value: WasmJwk) -> Self {
    value.0
  }
}

impl From<Jwk> for WasmJwk {
  fn from(value: Jwk) -> Self {
    WasmJwk::new(value)
  }
}

impl_wasm_json!(WasmJwk, Jwk);
impl_wasm_clone!(WasmJwk, Jwk);
