// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Result;
use crate::error::WasmResult;
use crate::verification::WasmMethodScope;
use identity_iota::document::verifiable::JwsVerificationOptions;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = JwsVerificationOptions, inspectable)]
pub struct WasmJwsVerificationOptions(pub(crate) JwsVerificationOptions);

#[wasm_bindgen(js_class = JwsVerificationOptions)]
impl WasmJwsVerificationOptions {
  /// Creates a new {@link JwsVerificationOptions} from the given fields.
  #[wasm_bindgen(constructor)]
  pub fn new(options: Option<IJwsVerificationOptions>) -> Result<WasmJwsVerificationOptions> {
    if let Some(options) = options {
      let options: JwsVerificationOptions = options.into_serde().wasm_result()?;
      Ok(WasmJwsVerificationOptions(options))
    } else {
      Ok(WasmJwsVerificationOptions(Default::default()))
    }
  }

  /// Set the expected value for the `nonce` parameter of the protected header.
  #[wasm_bindgen(js_name = setNonce)]
  pub fn set_nonce(&mut self, value: String) {
    self.0.nonce = Some(value);
  }

  /// Set the scope of the verification methods that may be used to verify the given JWS.
  #[wasm_bindgen(js_name = setScope)]
  pub fn set_scope(&mut self, value: &WasmMethodScope) {
    self.0.method_scope = Some(value.0);
  }
}

impl_wasm_json!(WasmJwsVerificationOptions, JwsVerificationOptions);
impl_wasm_clone!(WasmJwsVerificationOptions, JwsVerificationOptions);

/// Duck-typed interface to allow creating {@link JwsVerificationOptions} easily.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IJwsVerificationOptions")]
  pub type IJwsVerificationOptions;
}

#[wasm_bindgen(typescript_custom_section)]
const I_JWS_SIGNATURE_OPTIONS: &'static str = r#"
/** Holds options to create {@link JwsVerificationOptions}. */
interface IJwsVerificationOptions {
    /** 
    * A list of permitted extension parameters. 
    *
    * [More info](https://www.rfc-editor.org/rfc/rfc7515#section-4.1.11)
    */
    readonly crits?: [string];

    /** Verify that the `nonce` set in the protected header matches this.
     * 
     * [More Info](https://tools.ietf.org/html/rfc8555#section-6.5.2)
     */
    readonly nonce?: string;

    /** Verify the signing verification method relationship matches this.*/
    readonly methodScope?: MethodScope;
}"#;
