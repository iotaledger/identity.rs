// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::did::verifiable::VerifierOptions;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// Holds additional proof verification options.
/// See `IVerifierOptions`.
#[wasm_bindgen(js_name = VerifierOptions, inspectable)]
#[derive(Clone, Debug)]
pub struct WasmVerifierOptions(pub(crate) VerifierOptions);

#[wasm_bindgen(js_class = VerifierOptions)]
impl WasmVerifierOptions {
  /// Creates a new `VerifierOptions` from the given fields.
  ///
  /// Throws an error if any of the options are invalid.
  #[wasm_bindgen(constructor)]
  pub fn new(options: IVerifierOptions) -> Result<WasmVerifierOptions> {
    let options: VerifierOptions = options.into_serde().wasm_result()?;
    Ok(WasmVerifierOptions(options))
  }

  /// Creates a new `VerifierOptions` with default options.
  #[wasm_bindgen]
  pub fn default() -> WasmVerifierOptions {
    WasmVerifierOptions(VerifierOptions::default())
  }

  /// Serializes a `VerifierOptions` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `VerifierOptions` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmVerifierOptions> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl From<VerifierOptions> for WasmVerifierOptions {
  fn from(options: VerifierOptions) -> Self {
    WasmVerifierOptions(options)
  }
}

impl From<WasmVerifierOptions> for VerifierOptions {
  fn from(options: WasmVerifierOptions) -> Self {
    options.0
  }
}

impl_wasm_clone!(WasmVerifierOptions, VerifierOptions);

/// Duck-typed interface to allow creating `VerifierOptions` easily.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IVerifierOptions")]
  pub type IVerifierOptions;
}

#[wasm_bindgen(typescript_custom_section)]
const I_VERIFIER_OPTIONS: &'static str = r#"
/** Holds options to create a new `VerifierOptions`. */
interface IVerifierOptions {
    /** Verify the signing verification method relationship matches this.
    *
    * NOTE: `purpose` overrides the `method_scope` option.
    */
    readonly methodScope?: MethodScope;

    /** Verify the signing verification method type matches one specified.
    *
    * E.g. `[MethodType.Ed25519VerificationKey2018(), MethodType.X25519KeyAgreementKey2019()]`
    */
    readonly methodType?: Array<MethodType>;

    /** Verify the `Proof.challenge` field matches this. */
    readonly challenge?: string;

    /** Verify the `Proof.domain` field matches this. */
    readonly domain?: string;

    /** Verify the `Proof.purpose` field matches this. Also verifies that the signing
    * method has the corresponding verification method relationship.
    *
    * NOTE: `purpose` overrides the `method_scope` option.
    */
    readonly purpose?: ProofPurpose;

    /** Determines whether to error if the current time exceeds the `Proof.expires` field.
    *
    * Default: false (reject expired signatures).
    */
    readonly allowExpired?: boolean;
}"#;
