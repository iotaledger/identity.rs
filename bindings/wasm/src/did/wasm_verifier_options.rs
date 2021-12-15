use identity::did::verifiable::VerifierOptions;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// Holds additional signature verification options.
/// See `IVerifierOptions`.
#[wasm_bindgen(js_name = VerifierOptions)]
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
    readonly methodScope: MethodScope | undefined;

    /** Verify the signing verification method type matches one specified.
    *
    * E.g. `[MethodType.Ed25519VerificationKey2018(), MethodType.MerkleKeyCollection2021()]`
    */
    readonly methodType: Array<MethodType> | undefined;

    /** Verify the `Signature::challenge` field matches this. */
    readonly challenge: string | undefined;

    /** Verify the `Signature::domain` field matches this. */
    readonly domain: string | undefined;

    /** Verify the `Signature::purpose` field matches this. Also verifies that the signing
    * method has the corresponding verification method relationship.
    *
    * NOTE: `purpose` overrides the `method_scope` option.
    */
    readonly purpose: ProofPurpose | undefined;

    /** Determines whether to error if the current time exceeds the `Signature::expires` field.
    *
    * Default: false (reject expired signatures).
    */
    readonly allowExpired: boolean | undefined;
}"#;
