// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::credential::CredentialValidationOptions;
use identity_iota::credential::FailFast;
use identity_iota::credential::PresentationValidationOptions;
use identity_iota::credential::StatusCheck;
use identity_iota::credential::SubjectHolderRelationship;
use serde_repr::Deserialize_repr;
use serde_repr::Serialize_repr;
use wasm_bindgen::prelude::*;

use crate::error::Result;
use crate::error::WasmResult;

/// Options to declare validation criteria when validating credentials.
#[wasm_bindgen(js_name = CredentialValidationOptions)]
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
  #[allow(clippy::should_implement_trait)]
  #[wasm_bindgen]
  pub fn default() -> WasmCredentialValidationOptions {
    WasmCredentialValidationOptions::from(CredentialValidationOptions::default())
  }
}

impl_wasm_json!(WasmCredentialValidationOptions, CredentialValidationOptions);
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
  #[allow(clippy::should_implement_trait)]
  #[wasm_bindgen]
  pub fn default() -> WasmPresentationValidationOptions {
    WasmPresentationValidationOptions::from(PresentationValidationOptions::default())
  }
}

impl_wasm_json!(WasmPresentationValidationOptions, PresentationValidationOptions);
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

/// Controls validation behaviour when checking whether or not a credential has been revoked by its
/// [`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status).
#[wasm_bindgen(js_name = StatusCheck)]
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum WasmStatusCheck {
  /// Validate the status if supported, reject any unsupported
  /// [`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status) types.
  ///
  /// Only `RevocationBitmap2022` is currently supported.
  ///
  /// This is the default.
  Strict = 0,
  /// Validate the status if supported, skip any unsupported
  /// [`credentialStatus`](https://www.w3.org/TR/vc-data-model/#status) types.
  SkipUnsupported = 1,
  /// Skip all status checks.
  SkipAll = 2,
}

impl From<WasmStatusCheck> for StatusCheck {
  fn from(status_check: WasmStatusCheck) -> Self {
    match status_check {
      WasmStatusCheck::Strict => Self::Strict,
      WasmStatusCheck::SkipUnsupported => Self::SkipUnsupported,
      WasmStatusCheck::SkipAll => Self::SkipAll,
    }
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
    /** Declare that the credential is **not** considered valid if it expires before this `Timestamp`.
     * Uses the current datetime during validation if not set. */
    readonly earliestExpiryDate?: Timestamp;

    /** Declare that the credential is **not** considered valid if it was issued later than this `Timestamp`.
     * Uses the current datetime during validation if not set. */
    readonly latestIssuanceDate?: Timestamp;

    /** Validation behaviour for `credentialStatus`.
     *
     * Default: `StatusCheck.Strict`. */
    readonly status?: StatusCheck;

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
