// Copyright 2020-2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(typescript_custom_section)]
const T_CLAIM_PATH: &str = r#"
type ClaimPathSegment = string | number | null;
type ClaimPath = ClaimPathSegment[];
"#;

#[wasm_bindgen(typescript_custom_section)]
const T_DISCLOSABILITY: &str = r#"
type ClaimDisclosability = "always" | "allowed" | "never";
"#;

#[wasm_bindgen]
extern "C" {
  #[derive(Clone)]
  #[wasm_bindgen(typescript_type = ClaimPathSegment)]
  pub type WasmClaimPathSegment;

  #[derive(Clone)]
  #[wasm_bindgen(typescript_type = ClaimPath)]
  pub type WasmClaimPath;

  #[derive(Clone)]
  #[wasm_bindgen(typescript_type = ClaimDisclosability)]
  pub type WasmClaimDisclosability;
}

#[wasm_bindgen(js_name = ClaimMetadata, inspectable, getter_with_clone)]
pub struct WasmClaimMetadata {
  pub path: WasmClaimPath,
  pub display: Vec<WasmClaimDisplay>,
  pub sd: Option<WasmClaimDisclosability>,
  pub svg_id: Option<String>,
}

#[derive(Clone)]
#[wasm_bindgen(js_name = ClaimDisplay, inspectable, getter_with_clone)]
pub struct WasmClaimDisplay {
  /// A language tag as defined in [RFC5646](https://www.rfc-editor.org/rfc/rfc5646.txt).
  pub lang: String,
  /// A human-readable label for the claim.
  pub label: String,
  /// A human-readable description for the claim.
  pub description: Option<String>,
}
