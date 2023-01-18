// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::crypto::ProofOptions;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// Holds additional options for creating signatures.
/// See `IProofOptions`.
#[wasm_bindgen(js_name = ProofOptions)]
pub struct WasmProofOptions(pub(crate) ProofOptions);

#[wasm_bindgen(js_class = ProofOptions)]
impl WasmProofOptions {
  /// Creates a new `ProofOptions` from the given fields.
  ///
  /// Throws an error if any of the options are invalid.
  #[wasm_bindgen(constructor)]
  pub fn new(options: IProofOptions) -> Result<WasmProofOptions> {
    let proof_options: ProofOptions = options.into_serde().wasm_result()?;
    Ok(WasmProofOptions::from(proof_options))
  }

  /// Creates a new `ProofOptions` with default options.
  #[allow(clippy::should_implement_trait)]
  #[wasm_bindgen]
  pub fn default() -> WasmProofOptions {
    WasmProofOptions::from(ProofOptions::default())
  }
}

impl_wasm_json!(WasmProofOptions, ProofOptions);
impl_wasm_clone!(WasmProofOptions, ProofOptions);

impl From<ProofOptions> for WasmProofOptions {
  fn from(options: ProofOptions) -> Self {
    WasmProofOptions(options)
  }
}

impl From<WasmProofOptions> for ProofOptions {
  fn from(options: WasmProofOptions) -> Self {
    options.0
  }
}

/// Interface to allow creating `ProofOptions` easily.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IProofOptions")]
  pub type IProofOptions;
}

#[wasm_bindgen(typescript_custom_section)]
const I_PROOF_OPTIONS: &'static str = r#"
/** Holds options to create a new `ProofOptions`. */
interface IProofOptions {
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
