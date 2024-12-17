use std::sync::OnceLock;

use identity_iota::sd_jwt_rework::Hasher;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const I_HASHER: &str = r#"
interface Hasher {
  digest: (input: Uint8Array) => Uint8Array;
  algName: () => string;
  encodedDigest: (data: string) => string;
}
"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Hasher")]
  pub type WasmHasher;

  #[wasm_bindgen(structural, method)]
  pub fn digest(this: &WasmHasher, input: &[u8]) -> Vec<u8>;

  #[wasm_bindgen(structural, method, js_name = "algName")]
  pub fn alg_name(this: &WasmHasher) -> String;

  #[wasm_bindgen(structural, method, js_name = "encodedDigest")]
  pub fn encoded_digest(this: &WasmHasher, data: &str) -> String;
}

impl Hasher for WasmHasher {
  fn alg_name(&self) -> &str {
    static ALG: OnceLock<String> = OnceLock::new();
    ALG.get_or_init(|| self.alg_name())
  }

  fn digest(&self, input: &[u8]) -> Vec<u8> {
    self.digest(input)
  }

  fn encoded_digest(&self, disclosure: &str) -> String {
    self.encoded_digest(disclosure)
  }
}
