use identity_jose::jwk::{Jwk, JwkOperation, JwkParamsEc, JwkUse};
use wasm_bindgen::prelude::*;

use crate::common::ArrayString;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[wasm_bindgen(js_name = Jwk, inspectable)]
pub struct WasmJwk(pub(crate) Jwk);

#[wasm_bindgen(js_class = Jwk)]
impl WasmJwk {
  #[wasm_bindgen(constructor)]
  pub fn new(jwk: IJwk) -> Self {
    let jwk: Jwk = jwk.into_serde().unwrap();
    Self(jwk)
  }

  #[wasm_bindgen]
  pub fn kty(&self) -> String {
    self.0.kty().name().to_owned()
  }

  #[wasm_bindgen(js_name = use)]
  pub fn use_(&self) -> Option<String> {
    self.0.use_().as_ref().map(JwkUse::name).map(ToOwned::to_owned)
  }

  #[wasm_bindgen(js_name = keyOps)]
  pub fn key_ops(&self) -> ArrayString {
    self
      .0
      .key_ops()
      .unwrap_or_default()
      .iter()
      .map(JwkOperation::name)
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayString>()
  }

  #[wasm_bindgen]
  pub fn alg(&self) -> Option<String> {
    self.0.alg().map(ToOwned::to_owned)
  }

  #[wasm_bindgen]
  pub fn kid(&self) -> Option<String> {
    self.0.kid().map(ToOwned::to_owned)
  }

  #[wasm_bindgen]
  pub fn x5u(&self) -> Option<String> {
    self.0.x5u().map(AsRef::<str>::as_ref).map(ToOwned::to_owned)
  }

  #[wasm_bindgen]
  pub fn x5c(&self) -> ArrayString {
    self
      .0
      .x5c()
      .unwrap_or_default()
      .iter()
      .map(JsValue::from)
      .collect::<js_sys::Array>()
      .unchecked_into::<ArrayString>()
  }

  #[wasm_bindgen]
  pub fn x5t(&self) -> Option<String> {
    self.0.x5t().map(AsRef::<str>::as_ref).map(ToOwned::to_owned)
  }

  #[wasm_bindgen]
  pub fn x5t256(&self) -> Option<String> {
    self.0.x5t_s256().map(AsRef::<str>::as_ref).map(ToOwned::to_owned)
  }

  // #[wasm_bindgen(js_name = paramsEc)]
  // pub fn params_ec(&self) -> Option<String> {
  //   if let JwkParams::Ec(params_ec) = self.0.params() {
  //     Some("".to_owned())
  //   } else {
  //     None
  //   }
  // }
}

impl From<WasmJwk> for Jwk {
  fn from(value: WasmJwk) -> Self {
    value.0
  }
}

impl From<Jwk> for WasmJwk {
  fn from(value: Jwk) -> Self {
    WasmJwk(value)
  }
}

impl_wasm_json!(WasmJwk, Jwk);
impl_wasm_clone!(WasmJwk, Jwk);

#[wasm_bindgen]
pub struct WasmJwkParamsEc(JwkParamsEc);

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IJwk")]
  pub type IJwk;
}

// #[derive(Deserialize)]
// #[serde(rename_all = "camelCase")]
// #[typescript(name = "IJwk", readonly, optional)]
// #[allow(non_snake_case, dead_code)]
// struct IJwkHelper {
//   #[typescript(type = "string")]
//   kty: Option<JwkType>,

//   #[typescript(type = "string")]
//   r#use: Option<String>,

//   #[typescript(type = "string[]")]
//   key_ops: Vec<String>,

//   #[typescript(type = "string")]
//   alg: Option<String>,

//   #[typescript(type = "string")]
//   kid: Option<String>,

//   #[typescript(type = "string")]
//   x5u: Option<String>,
//   #[typescript(type = "string[]")]
//   x5c: Vec<String>,
//   #[typescript(type = "string")]
//   x5t: Option<String>,
//   #[typescript(type = "string")]
//   x5t_S256: Option<String>,
//   #[typescript(type = "string")]
//   crv: Option<String>,
//   #[typescript(type = "string")]
//   d: Option<String>,
//   #[typescript(type = "string")]
//   dp: Option<String>,
//   #[typescript(type = "string")]
//   dq: Option<String>,
//   #[typescript(type = "string")]
//   e: Option<String>,
//   #[typescript(type = "string")]
//   k: Option<String>,
//   #[typescript(type = "string")]
//   n: Option<String>,
//   #[typescript(type = "Array<{d?:string,r?:string,t?:string}>")]
//   oth: Option<Vec<JwkParamsRsaPrime>>,
//   #[typescript(type = "string")]
//   p: Option<String>,
//   #[typescript(type = "string")]
//   q: Option<String>,
//   #[typescript(type = "string")]
//   qi: Option<String>,
//   #[typescript(type = "string")]
//   x: Option<String>,
//   #[typescript(type = "string")]
//   y: Option<String>,
// }

#[wasm_bindgen(typescript_custom_section)]
const I_JWK: &'static str = r#"
/** A JSON Web Key. */
export interface IJwk {
  /** Key Type.

  Identifies the cryptographic algorithm family used with the key.
  
  [More Info](https://tools.ietf.org/html/rfc7517#section-4.1) */
  kty: string
  /** Public Key Use.
  
  Identifies the intended use of the public key.
  
  [More Info](https://tools.ietf.org/html/rfc7517#section-4.2) */
  use?: string
  /** Key Operations.
 
  Identifies the operation(s) for which the key is intended to be used.
 
  [More Info](https://tools.ietf.org/html/rfc7517#section-4.3) */
  key_ops?: string[]
  /** Algorithm.
 
  Identifies the algorithm intended for use with the key.
 
  [More Info](https://tools.ietf.org/html/rfc7517#section-4.4) */
  alg?: JwsAlgorithm
  /** Key ID.
 
  Used to match a specific key among a set of keys within a JWK Set.
 
  [More Info](https://tools.ietf.org/html/rfc7517#section-4.5) */
  kid?: string
  /** X.509 URL.
 
  A URI that refers to a resource for an X.509 public key certificate or
  certificate chain.
  
  [More Info](https://tools.ietf.org/html/rfc7517#section-4.6) */
  x5u?: string
  /** X.509 Certificate Chain.
 
  Contains a chain of one or more PKIX certificates.
 
  [More Info](https://tools.ietf.org/html/rfc7517#section-4.7) */
  x5c?: string[]
  /** X.509 Certificate SHA-1 Thumbprint.

  A base64url-encoded SHA-1 thumbprint of the DER encoding of an X.509
  certificate.

  [More Info](https://tools.ietf.org/html/rfc7517#section-4.8) */
  x5t?: string
  /** X.509 Certificate SHA-256 Thumbprint.
 
  A base64url-encoded SHA-256 thumbprint of the DER encoding of an X.509
  certificate.
 
  [More Info](https://tools.ietf.org/html/rfc7517#section-4.9) */
  'x5t#S256'?: string
  crv?: string
  d?: string
  dp?: string
  dq?: string
  e?: string
  k?: string
  n?: string
  oth?: Array<{
    d?: string
    r?: string
    t?: string
  }>
  p?: string
  q?: string
  qi?: string
  x?: string
  y?: string
}
"#;
