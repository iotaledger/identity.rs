// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::crypto::SignatureOptions;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// Holds additional options for creating signatures.
/// See `ISignatureOptions`.
#[wasm_bindgen(js_name = SignatureOptions)]
#[derive(Clone, Debug)]
pub struct WasmSignatureOptions(pub(crate) SignatureOptions);

#[wasm_bindgen(js_class = SignatureOptions)]
impl WasmSignatureOptions {
  /// Creates a new `SignatureOptions` from the given fields.
  ///
  /// Throws an error if any of the options are invalid.
  #[wasm_bindgen(constructor)]
  pub fn new(options: ISignatureOptions) -> Result<WasmSignatureOptions> {
    let signature_options: SignatureOptions = options.into_serde().wasm_result()?;
    Ok(WasmSignatureOptions::from(signature_options))
  }

  /// Creates a new `SignatureOptions` with default options.
  #[wasm_bindgen]
  pub fn default() -> WasmSignatureOptions {
    WasmSignatureOptions::from(SignatureOptions::default())
  }
}

impl_wasm_clone!(WasmSignatureOptions, SignatureOptions);

impl From<SignatureOptions> for WasmSignatureOptions {
  fn from(options: SignatureOptions) -> Self {
    WasmSignatureOptions(options)
  }
}

impl From<WasmSignatureOptions> for SignatureOptions {
  fn from(options: WasmSignatureOptions) -> Self {
    options.0
  }
}

/// Interface to allow creating `SignatureOptions` easily.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "ISignatureOptions")]
  pub type ISignatureOptions;
}

#[wasm_bindgen(typescript_custom_section)]
const I_SIGNATURE_OPTIONS: &'static str = r#"
/** Holds options to create a new `SignatureOptions`. */
interface ISignatureOptions {
    /** When the proof was generated. */
    readonly created?: Timestamp;

    /** When the proof expires. */
    readonly expires?: Timestamp;

    /** Challenge from a proof requester to mitigate replay attacks. */
    readonly challenge?: string;

    /** Domain for which a proof is valid to mitigate replay attacks. */
    readonly domain?: string;

    /** Purpose for which the proof was generated. */
    readonly purpose?: ProofPurpose;
}"#;
