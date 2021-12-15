use core::str::FromStr;

use identity::crypto::ProofPurpose;
use identity::did::verifiable::VerifierOptions;
use identity::did::MethodScope;
use identity::did::MethodType;
use serde;
use serde::Deserialize;
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
    // TODO: see if we can just re-use VerifierOptions deserializer, only problem is method_type?
    //       Could expose the enum with MethodType.Ed25519VerificationKey2018 etc.?
    #[derive(Deserialize)]
    struct IVerifierOptionsDeserializer {
      #[serde(rename = "methodScope")]
      method_scope: Option<String>,
      #[serde(rename = "methodType")]
      method_type: Option<Vec<String>>,
      challenge: Option<String>,
      domain: Option<String>,
      purpose: Option<String>,
      #[serde(rename = "allowExpired")]
      allow_expired: Option<bool>,
    }
    let options: IVerifierOptionsDeserializer = options.into_serde().wasm_result()?;

    let mut this: VerifierOptions = VerifierOptions::new();
    this.method_scope = options
      .method_scope
      .as_deref()
      .map(MethodScope::from_str)
      .transpose()
      .wasm_result()?;
    this.method_type = options
      .method_type
      .map(|method_type| {
        method_type
          .into_iter()
          .map(|method_type| MethodType::from_str(&method_type))
          .collect::<std::result::Result<_, _>>()
      })
      .transpose()
      .wasm_result()?;
    this.domain = options.domain;
    this.challenge = options.challenge;
    this.purpose = options
      .purpose
      .as_deref()
      .map(ProofPurpose::from_str)
      .transpose()
      .wasm_result()?;
    this.allow_expired = options.allow_expired;
    Ok(WasmVerifierOptions::from(this))
  }

  /// Creates a new `VerifierOptions` with default options.
  #[wasm_bindgen]
  pub fn default() -> WasmVerifierOptions {
    WasmVerifierOptions::from(VerifierOptions::default())
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

  // TODO: remove? Alternative to IVerifierOptionsDeserializer
  // #[serde(structural, getter = methodScope)]
  // pub fn method_scope(this: &IVerifierOptions) -> Option<String>;
  // #[serde(structural, getter = methodType)]
  // pub fn method_type(this: &IVerifierOptions) -> Option<Vec<String>>;
  // #[serde(structural, getter)]
  // pub fn challenge(this: &IVerifierOptions) -> Option<String>;
  // #[serde(structural, getter)]
  // pub fn domain(this: &IVerifierOptions) -> Option<String>;
  // #[serde(structural, getter)]
  // pub fn purpose(this: &IVerifierOptions) -> Option<String>;
  // #[serde(structural, getter = allowExpired)]
  // pub fn allow_expired(this: &IVerifierOptions) -> Option<bool>;
}

#[wasm_bindgen(typescript_custom_section)]
const I_VERIFIER_OPTIONS: &'static str = r#"
/** Holds options to create a new `VerifierOptions`. */
interface IVerifierOptions {
    /** Verify the signing verification method relationship matches this.
    *
    * E.g. "Authentication", "CapabilityInvocation", etc.
    * NOTE: `purpose` overrides the `method_scope` option.
    */
    readonly methodScope: string | undefined;

    /** Verify the signing verification method type matches one specified.
    *
    * E.g. ["Ed25519VerificationKey2018", "MerkleKeyCollection2021"]
    */
    readonly methodType: Array<string> | undefined;

    /** Verify the `Signature::challenge` field matches this. */
    readonly challenge: string | undefined;

    /** Verify the `Signature::domain` field matches this. */
    readonly domain: string | undefined;

    /** Verify the `Signature::purpose` field matches this. Also verifies that the signing
    * method has the corresponding verification method relationship.
    *
    * Only "authentication" and "assertionMethod" are allowed.
    *
    * NOTE: `purpose` overrides the `method_scope` option.
    */
    readonly purpose: "authentication" | "assertionMethod" | undefined;

    /** Determines whether to error if the current time exceeds the `Signature::expires` field.
    *
    * Default: false (reject expired signatures).
    */
    readonly allowExpired: boolean | undefined;
}"#;
