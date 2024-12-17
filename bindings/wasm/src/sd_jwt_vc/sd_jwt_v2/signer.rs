use async_trait::async_trait;
use identity_iota::sd_jwt_rework::JsonObject;
use identity_iota::sd_jwt_rework::JwsSigner;
use js_sys::Error as JsError;
use js_sys::Object;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;

use crate::error::Result;

#[wasm_bindgen(typescript_custom_section)]
const I_JWS_SIGNER: &str = r#"
interface JwsSigner {
  sign: (header: any, payload: any) => Promise<Uint8Array>;
}
"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "JwsSigner")]
  pub type WasmJwsSigner;

  #[wasm_bindgen(structural, method, catch)]
  pub async fn sign(this: &WasmJwsSigner, header: Object, payload: Object) -> Result<Uint8Array>;
}

#[async_trait(?Send)]
impl JwsSigner for WasmJwsSigner {
  type Error = String;
  async fn sign(&self, header: &JsonObject, payload: &JsonObject) -> std::result::Result<Vec<u8>, Self::Error> {
    let header = serde_wasm_bindgen::to_value(header)
      .map_err(|e| format!("{e:?}"))?
      .unchecked_into();
    let payload = serde_wasm_bindgen::to_value(payload)
      .map_err(|e| format!("{e:?}"))?
      .unchecked_into();

    self
      .sign(header, payload)
      .await
      .map_err(|e| e.unchecked_into::<JsError>().to_string().into())
      .map(|arr| arr.to_vec())
  }
}
