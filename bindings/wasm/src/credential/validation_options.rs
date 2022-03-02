// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity::iota::CredentialValidationOptions;
use identity::iota::FailFast;
use identity::iota::PresentationValidationOptions;
use identity::iota::SubjectHolderRelationship;
//use identity::iota::PresentationValidationOptions;
use crate::error::Result;
use crate::error::WasmResult;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = CredentialValidationOptions)]
#[derive(Clone, Debug)]
/// Options to declare validation criteria when validating credentials.
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
}

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
#[derive(Clone, Debug)]
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
}

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

/// Declares how a credential subject must relate to the presentation holder.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[wasm_bindgen(js_name = SubjectHolderRelationship)]
#[non_exhaustive]
pub enum WasmSubjectHolderRelationship {
  /// Declare that the holder must always match the subject.
  AlwaysSubject = 0,
  /// Declare that the holder must match the subject on credentials with the nonTransferable property set.
  SubjectOnNonTransferable = 1,
  /// Declares that the subject is not required to have any kind of relationship to the holder.  
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
    /** Declare that the credential is **not** considered valid if it expires before this */
    readonly earliestExpiryDate?: Timestamp;

    /** Declare that the credential is **not** considered valid if it was issued later than this */
    readonly latestIssuanceDate?: Timestamp;

    /** Declare that the credential's signature must be verified according to these `VerifierOptions`. */
    readonly verifierOptions?: VerifierOptions;

}"#;

#[wasm_bindgen(typescript_custom_section)]
const I_PRESENTATION_VALIDATION_OPTIONS: &'static str = r#"
/** Holds options to create a new `PresentationValidationOptions`. */
interface IPresentationValidationOptions {
    /** Declare that the credentials of the presentation must all be validated according to these `CredentialValidationOptions`. */
    readonly sharedValidationOptions?: CredentialValidationOptions;

    /** Declare that the presentation's signature is to be verified according to these `VerifierOptions`. */
    readonly presentationVerifierOptions?: VerifierOptions;

    /** Declare how the presentation's credential subjects must relate to the holder. */
    readonly subjectHolderRelationship?: SubjectHolderRelationship;

}"#;

/// Declares when validation should return with an error.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[wasm_bindgen(js_name = FailFast)]
pub enum WasmFailFast {
  /// Declare that validation should fail after all errors have been found
  No = 0,
  /// Declare that validation must fail upon the first error is found
  Yes = 1,
}

impl From<WasmFailFast> for FailFast {
  fn from(fail_fast: WasmFailFast) -> Self {
    match fail_fast {
      WasmFailFast::No => Self::No,
      WasmFailFast::Yes => Self::Yes,
    }
  }
}
