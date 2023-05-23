// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Result;
use crate::error::WasmResult;
use identity_iota::core::Url;
use identity_iota::storage::JwsOptions;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = JwsOptions, inspectable)]
pub struct WasmJwsOptions(pub(crate) JwsOptions);

#[wasm_bindgen(js_class = JwsOptions)]
impl WasmJwsOptions {
  #[wasm_bindgen(constructor)]
  pub fn new(options: Option<IJwsOptions>) -> Result<WasmJwsOptions> {
    if let Some(options) = options {
      let options: JwsOptions = options.into_serde().wasm_result()?;
      Ok(WasmJwsOptions(options))
    } else {
      Ok(WasmJwsOptions(Default::default()))
    }
  }

  /// Replace the value of the `attachJwk` field.
  #[wasm_bindgen(js_name = setAttachJwk)]
  pub fn set_attach_jwk(&mut self, value: bool) {
    self.0.attach_jwk = value;
  }

  /// Replace the value of the `b64` field.
  #[wasm_bindgen(js_name = setB64)]
  pub fn set_b64(&mut self, value: bool) {
    self.0.b64 = Some(value);
  }

  /// Replace the value of the `typ` field.
  #[wasm_bindgen(js_name = setTyp)]
  pub fn set_typ(&mut self, value: String) {
    self.0.typ = Some(value);
  }

  /// Replace the value of the `cty` field.
  #[wasm_bindgen(js_name = setCty)]
  pub fn set_cty(&mut self, value: String) {
    self.0.cty = Some(value);
  }

  /// Replace the value of the `url` field.
  #[wasm_bindgen(js_name = serUrl)]
  pub fn set_url(&mut self, value: String) -> Result<()> {
    self.0.url = Some(Url::parse(value).wasm_result()?);
    Ok(())
  }

  /// Replace the value of the `nonce` field.
  #[wasm_bindgen(js_name = setNonce)]
  pub fn set_nonce(&mut self, value: String) {
    self.0.nonce = Some(value);
  }

  /// Replace the value of the `detached_payload` field.
  #[wasm_bindgen(js_name = setDetachedPayload)]
  pub fn set_detached_payload(&mut self, value: bool) {
    self.0.detached_payload = value;
  }
}

impl_wasm_json!(WasmJwsOptions, JwsOptions);
impl_wasm_clone!(WasmJwsOptions, JwsOptions);

/// Duck-typed interface to allow creating `JwsOptions` easily.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IJwsOptions")]
  pub type IJwsOptions;
}

#[wasm_bindgen(typescript_custom_section)]
const I_JWS_SIGNATURE_OPTIONS: &'static str = r#"
/** Holds options to create `JwsOptions`. */
interface IJwsOptions {
    /** Whether to attach the public key in the corresponding method
     * to the JWS header.
     * 
     * Default: false
     */
    readonly attachJwk?: boolean;

    /** Whether to Base64url encode the payload or not.
    *
    * [More Info](https://tools.ietf.org/html/rfc7797#section-3)
    */
    readonly b64?: boolean;

    /** The Type value to be placed in the protected header.
     * 
     * [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.9)
    */
    readonly typ?: string;

    /** Content Type to be placed in the protected header.
     * 
     * [More Info](https://tools.ietf.org/html/rfc7515#section-4.1.10)
     */
    readonly cty?: string;

    /** The URL to be placed in the protected header.
     * 
     * [More Info](https://tools.ietf.org/html/rfc8555#section-6.4.1)
     */
    readonly url?: string;

    /** The nonce to be placed in the protected header.
     * 
     * [More Info](https://tools.ietf.org/html/rfc8555#section-6.5.2)
     */
    readonly nonce?: string;

    /**   /// Whether the payload should be detached from the JWS.
     * 
     * [More Info](https://www.rfc-editor.org/rfc/rfc7515#appendix-F).
     */
    readonly detachedPayload?: boolean
}"#;
