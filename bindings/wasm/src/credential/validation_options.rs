// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Result;
use crate::error::WasmResult;
use identity::iota::CredentialValidationOptions;
use identity::iota::FailFast;
use identity::iota::PresentationValidationOptions;
use identity::iota::SubjectHolderRelationship;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;
use wasm_bindgen::prelude::*;

/// Options to declare validation criteria when validating credentials.
#[wasm_bindgen(js_name = CredentialValidationOptions)]
#[derive(Clone)]
pub struct WasmCredentialValidationOptions(pub(crate) CredentialValidationOptions);

#[wasm_bindgen(js_class = CredentialValidationOptions)]
impl WasmCredentialValidationOptions {
  /// Creates a new `CredentialValidationOptions` from the given fields.
  ///
  /// Throws an error if any of the options are invalid.
  #[wasm_bindgen(constructor)]
  pub fn new(options: ICredentialValidationOptions) -> Result<WasmCredentialValidationOptions> {
    let options: CredentialValidationOptions = options.into_serde().wasm_result()?;
    Ok(WasmCredentialValidationOptions::from(options))
  }

  /// Creates a new `CredentialValidationOptions` with defaults.
  #[wasm_bindgen]
  pub fn default() -> WasmCredentialValidationOptions {
    WasmCredentialValidationOptions::from(CredentialValidationOptions::default())
  }

  /// Serializes a `CredentialValidationOptions` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `CredentialValidationOptions` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmCredentialValidationOptions> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl_wasm_clone!(WasmCredentialValidationOptions, CredentialValidationOptions);

impl From<CredentialValidationOptions> for WasmCredentialValidationOptions {
  fn from(options: CredentialValidationOptions) -> Self {
    Self(options)
  }
}

impl From<WasmCredentialValidationOptions> for CredentialValidationOptions {
  fn from(options: WasmCredentialValidationOptions) -> Self {
    options.0
  }
}

/// Options to declare validation criteria when validating presentation.
#[wasm_bindgen(js_name = PresentationValidationOptions)]
#[derive(Clone)]
pub struct WasmPresentationValidationOptions(pub(crate) PresentationValidationOptions);

#[wasm_bindgen(js_class = PresentationValidationOptions)]
impl WasmPresentationValidationOptions {
  /// Creates a new `PresentationValidationOptions` from the given fields.
  ///
  /// Throws an error if any of the options are invalid.
  #[wasm_bindgen(constructor)]
  pub fn new(options: IPresentationValidationOptions) -> Result<WasmPresentationValidationOptions> {
    let options: PresentationValidationOptions = options.into_serde().wasm_result()?;
    Ok(WasmPresentationValidationOptions::from(options))
  }

  /// Creates a new `PresentationValidationOptions` with defaults.
  #[wasm_bindgen]
  pub fn default() -> WasmPresentationValidationOptions {
    WasmPresentationValidationOptions::from(PresentationValidationOptions::default())
  }

  /// Serializes a `PresentationValidationOptions` as a JSON object.
  #[wasm_bindgen(js_name = toJSON)]
  pub fn to_json(&self) -> Result<JsValue> {
    JsValue::from_serde(&self.0).wasm_result()
  }

  /// Deserializes a `PresentationValidationOptions` from a JSON object.
  #[wasm_bindgen(js_name = fromJSON)]
  pub fn from_json(json: &JsValue) -> Result<WasmPresentationValidationOptions> {
    json.into_serde().map(Self).wasm_result()
  }
}

impl_wasm_clone!(WasmPresentationValidationOptions, PresentationValidationOptions);

impl From<PresentationValidationOptions> for WasmPresentationValidationOptions {
  fn from(options: PresentationValidationOptions) -> Self {
    Self(options)
  }
}

impl From<WasmPresentationValidationOptions> for PresentationValidationOptions {
  fn from(options: WasmPresentationValidationOptions) -> Self {
    options.0
  }
}

/// Declares how credential subjects must relate to the presentation holder during validation.
/// See `PresentationValidationOptions::subject_holder_relationship`.
///
/// See also the [Subject-Holder Relationship](https://www.w3.org/TR/vc-data-model/#subject-holder-relationships) section of the specification.
#[wasm_bindgen(js_name = SubjectHolderRelationship)]
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum WasmSubjectHolderRelationship {
  /// The holder must always match the subject on all credentials, regardless of their [`nonTransferable`](https://www.w3.org/TR/vc-data-model/#nontransferable-property) property.
  /// This variant is the default used if no other variant is specified when constructing a new
  /// `PresentationValidationOptions`.
  AlwaysSubject = 0,
  /// The holder must match the subject only for credentials where the [`nonTransferable`](https://www.w3.org/TR/vc-data-model/#nontransferable-property) property is `true`.
  SubjectOnNonTransferable = 1,
  /// The holder is not required to have any kind of relationship to any credential subject.
  Any = 2,
}

impl From<WasmSubjectHolderRelationship> for SubjectHolderRelationship {
  fn from(subject_holder_relationship: WasmSubjectHolderRelationship) -> Self {
    match subject_holder_relationship {
      WasmSubjectHolderRelationship::AlwaysSubject => Self::AlwaysSubject,
      WasmSubjectHolderRelationship::SubjectOnNonTransferable => Self::SubjectOnNonTransferable,
      WasmSubjectHolderRelationship::Any => Self::Any,
    }
  }
}

// Interface to allow creating `CredentialValidationOptions` easily.
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "ICredentialValidationOptions")]
  pub type ICredentialValidationOptions;

  #[wasm_bindgen(typescript_type = "IPresentationValidationOptions")]
  pub type IPresentationValidationOptions;
}

#[wasm_bindgen(typescript_custom_section)]
const I_CREDENTIAL_VALIDATION_OPTIONS: &'static str = r#"
/** Holds options to create a new `CredentialValidationOptions`. */
interface ICredentialValidationOptions {
    /**  Declare that the credential is **not** considered valid if it expires before this `Timestamp`.
     * Uses the current datetime during validation if not set. */
    readonly earliestExpiryDate?: Timestamp;

    /** Declare that the credential is **not** considered valid if it was issued later than this `Timestamp`.
     * Uses the current datetime during validation if not set. */
    readonly latestIssuanceDate?: Timestamp;

    /** Options which affect the verification of the signature on the credential. */
    readonly verifierOptions?: VerifierOptions;

}"#;

#[wasm_bindgen(typescript_custom_section)]
const I_PRESENTATION_VALIDATION_OPTIONS: &'static str = r#"
/** Holds options to create a new `PresentationValidationOptions`. */
interface IPresentationValidationOptions {
    /** Declare that the credentials of the presentation must all be validated according to these `CredentialValidationOptions`. */
    readonly sharedValidationOptions?: CredentialValidationOptions;

    /** Options which affect the verification of the signature on the presentation. */
    readonly presentationVerifierOptions?: VerifierOptions;

    /** Declare how the presentation's credential subjects must relate to the holder.
     * 
     * Default: SubjectHolderRelationship.AlwaysSubject
     */
    readonly subjectHolderRelationship?: SubjectHolderRelationship;

}"#;

/// Declares when validation should return if an error occurs.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[wasm_bindgen(js_name = FailFast)]
pub enum WasmFailFast {
  /// Return all errors that occur during validation.
  AllErrors = 0,
  /// Return after the first error occurs.
  FirstError = 1,
}

impl From<WasmFailFast> for FailFast {
  fn from(fail_fast: WasmFailFast) -> Self {
    match fail_fast {
      WasmFailFast::AllErrors => Self::AllErrors,
      WasmFailFast::FirstError => Self::FirstError,
    }
  }
}
