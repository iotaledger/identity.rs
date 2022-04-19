// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "Evidence")]
  pub type Evidence;

  #[wasm_bindgen(typescript_type = "Issuer")]
  pub type Issuer;

  #[wasm_bindgen(typescript_type = "Policy")]
  pub type Policy;

  #[wasm_bindgen(typescript_type = "RefreshService")]
  pub type RefreshService;
}

#[wasm_bindgen(typescript_custom_section)]
const I_EVIDENCE: &'static str = r#"
/** Information used to increase confidence in the claims of a {@link Credential}.

[More Info](https://www.w3.org/TR/vc-data-model/#evidence) */
interface Evidence {
  /** A URL that allows retrieval of information about the evidence. */
  readonly id?: string;
  /** The type(s) of the credential evidence. */
  readonly types: string | Array<string>;
  /** Additional properties of the credential evidence. */
  readonly [properties: string | symbol]: unknown;
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const I_ISSUER: &'static str = r#"
/** An identifier representing the issuer of a {@link Credential}.

[More Info](https://www.w3.org/TR/vc-data-model/#issuer) */
interface Issuer {
  /** A URL identifying the credential issuer. */
  readonly id?: string;
  /** Additional properties of the credential issuer. */
  readonly [properties: string | symbol]: unknown;
}"#;

#[wasm_bindgen(typescript_custom_section)]
const I_POLICY: &'static str = r#"
/** Information used to express obligations, prohibitions, and permissions about a {@link Credential} or {@link Presentation}.

[More Info](https://www.w3.org/TR/vc-data-model/#terms-of-use) */
interface Policy {
  /** A URL identifying the credential terms-of-use. */
  readonly id?: string;
  /** The type(s) of the credential terms-of-use. */
  readonly types: string | Array<string>;
  /** Additional properties of the credential terms-of-use. */
  readonly [properties: string | symbol]: unknown;
}"#;

#[wasm_bindgen(typescript_custom_section)]
const I_REFRESH_SERVICE: &'static str = r#"
/** Information used to refresh or assert the status of a {@link Credential}.

[More Info](https://www.w3.org/TR/vc-data-model/#refreshing) */
interface RefreshService {
  /** The URL of the credential refresh service. */
  readonly id: string;
  /** The type(s) of the credential refresh service. */
  readonly types: string | Array<string>;
  /** Additional properties of the credential refresh service. */
  readonly [properties: string | symbol]: unknown;
}"#;
