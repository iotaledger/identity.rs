use identity_jose::jws::JwsAlgorithm;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = JwsAlgorithm)]
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[repr(u8)]
pub enum WasmJwsAlgorithm {
  /// HMAC using SHA-256
  HS256 = 0,
  /// HMAC using SHA-384
  HS384 = 1,
  /// HMAC using SHA-512
  HS512 = 2,
  /// RSASSA-PKCS1-v1_5 using SHA-256
  RS256 = 3,
  /// RSASSA-PKCS1-v1_5 using SHA-384
  RS384 = 4,
  /// RSASSA-PKCS1-v1_5 using SHA-512
  RS512 = 5,
  /// RSASSA-PSS using SHA-256 and MGF1 with SHA-256
  PS256 = 6,
  /// RSASSA-PSS using SHA-384 and MGF1 with SHA-384
  PS384 = 7,
  /// RSASSA-PSS using SHA-512 and MGF1 with SHA-512
  PS512 = 8,
  /// ECDSA using P-256 and SHA-256
  ES256 = 9,
  /// ECDSA using P-384 and SHA-384
  ES384 = 10,
  /// ECDSA using P-521 and SHA-512
  ES512 = 11,
  /// ECDSA using secp256k1 curve and SHA-256
  ES256K = 12,
  #[serde(rename = "none")]
  /// No digital signature or MAC performed
  NONE = 13,
  /// EdDSA signature algorithms
  EdDSA = 14,
}

impl From<WasmJwsAlgorithm> for JwsAlgorithm {
  fn from(value: WasmJwsAlgorithm) -> Self {
    match value {
      WasmJwsAlgorithm::HS256 => JwsAlgorithm::HS256,
      WasmJwsAlgorithm::HS384 => JwsAlgorithm::HS384,
      WasmJwsAlgorithm::HS512 => JwsAlgorithm::HS512,
      WasmJwsAlgorithm::RS256 => JwsAlgorithm::RS256,
      WasmJwsAlgorithm::RS384 => JwsAlgorithm::RS384,
      WasmJwsAlgorithm::RS512 => JwsAlgorithm::RS512,
      WasmJwsAlgorithm::PS256 => JwsAlgorithm::PS256,
      WasmJwsAlgorithm::PS384 => JwsAlgorithm::PS384,
      WasmJwsAlgorithm::PS512 => JwsAlgorithm::PS512,
      WasmJwsAlgorithm::ES256 => JwsAlgorithm::ES256,
      WasmJwsAlgorithm::ES384 => JwsAlgorithm::ES384,
      WasmJwsAlgorithm::ES512 => JwsAlgorithm::ES512,
      WasmJwsAlgorithm::ES256K => JwsAlgorithm::ES256K,
      WasmJwsAlgorithm::NONE => JwsAlgorithm::NONE,
      WasmJwsAlgorithm::EdDSA => JwsAlgorithm::EdDSA,
    }
  }
}

impl From<JwsAlgorithm> for WasmJwsAlgorithm {
  fn from(value: JwsAlgorithm) -> Self {
    match value {
      JwsAlgorithm::HS256 => WasmJwsAlgorithm::HS256,
      JwsAlgorithm::HS384 => WasmJwsAlgorithm::HS384,
      JwsAlgorithm::HS512 => WasmJwsAlgorithm::HS512,
      JwsAlgorithm::RS256 => WasmJwsAlgorithm::RS256,
      JwsAlgorithm::RS384 => WasmJwsAlgorithm::RS384,
      JwsAlgorithm::RS512 => WasmJwsAlgorithm::RS512,
      JwsAlgorithm::PS256 => WasmJwsAlgorithm::PS256,
      JwsAlgorithm::PS384 => WasmJwsAlgorithm::PS384,
      JwsAlgorithm::PS512 => WasmJwsAlgorithm::PS512,
      JwsAlgorithm::ES256 => WasmJwsAlgorithm::ES256,
      JwsAlgorithm::ES384 => WasmJwsAlgorithm::ES384,
      JwsAlgorithm::ES512 => WasmJwsAlgorithm::ES512,
      JwsAlgorithm::ES256K => WasmJwsAlgorithm::ES256K,
      JwsAlgorithm::NONE => WasmJwsAlgorithm::NONE,
      JwsAlgorithm::EdDSA => WasmJwsAlgorithm::EdDSA,
    }
  }
}

// impl_wasm_json!(WasmJwsAlgorithm, JwsAlgorithm);
// impl_wasm_clone!(WasmJwsAlgorithm, JwsAlgorithm);
