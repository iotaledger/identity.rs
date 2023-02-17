use identity_iota::core::ToJson;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn takes_jwk(jwk: IJwk) -> String {
  let jwk: identity_jose::jwk::Jwk = jwk.into_serde().expect("should work");
  jwk.to_json_pretty().unwrap()
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IJwk")]
  pub type IJwk;
}

#[wasm_bindgen(typescript_custom_section)]
const I_JWK: &'static str = r#"
export interface IJwk {
  alg?: JwsAlgorithm
  kty: string
}
"#;
