// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::WasmCredential;
use super::WasmCredentialValidationOptions;
use super::WasmSubjectHolderRelationship;

use crate::common::WasmTimestamp;
use crate::common::WasmUrl;
use crate::credential::validation_options::WasmFailFast;
use crate::did::ArrayResolvedDocument;
use crate::did::WasmResolvedDocument;
use crate::did::WasmVerifierOptions;
use crate::error::Result;
use crate::error::WasmResult;
use identity::iota::CredentialValidator;
use identity::iota::ResolvedIotaDocument;
use identity::iota::ValidationError;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = CredentialValidator, inspectable)]
#[derive(Clone, Debug)]
pub struct WasmCredentialValidator;

#[wasm_bindgen(js_class = CredentialValidator)]
impl WasmCredentialValidator {
  /// Validates a `Credential`.
  ///
  /// The following properties are validated according to `options`:
  /// - The issuer's signature,
  /// - The expiration date,
  /// - The issuance date
  /// - The semantic structure.
  ///
  /// ### Warning
  ///  There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
  /// `credentialStatus`, `type`, `credentialSchema`, `refreshService`, **and more**.
  /// These should be manually checked after validation, according to your requirements.
  ///
  /// ### Errors
  /// An error is returned whenever a validated condition is not satisfied.
  #[wasm_bindgen]
  pub fn validate(
    credential: &WasmCredential,
    issuer: &WasmResolvedDocument,
    options: &WasmCredentialValidationOptions,
    fail_fast: WasmFailFast,
  ) -> Result<()> {
    CredentialValidator::validate(&credential.0, &issuer.0, &options.0, fail_fast.into()).wasm_result()
  }

  /// Validates the semantic structure of the `Credential`.
  ///
  /// ### Warning
  /// This does not validate against the credential's schema nor the structure of the subject claims.
  #[wasm_bindgen(js_name = checkStructure)]
  pub fn check_structure(credential: &WasmCredential) -> Result<()> {
    credential
      .0
      .check_structure()
      .map_err(ValidationError::CredentialStructure)
      .wasm_result()
  }

  /// Validate that the credential expires on or after the specified timestamp.
  #[wasm_bindgen(js_name = checkExpiresOnOrAfter)]
  pub fn check_expires_on_or_after(credential: &WasmCredential, timestamp: WasmTimestamp) -> Result<()> {
    CredentialValidator::check_expires_on_or_after(&credential.0, timestamp.0).wasm_result()
  }

  /// Validate that the credential is issued on or before the specified timestamp.
  #[wasm_bindgen(js_name = checkIssuedOnOrBefore)]
  pub fn check_issued_on_or_before(credential: &WasmCredential, timestamp: WasmTimestamp) -> Result<()> {
    CredentialValidator::check_issued_on_or_before(&credential.0, timestamp.0).wasm_result()
  }

  /// Verify the signature using the DID Document of a trusted issuer.
  ///
  /// ### Errors
  /// This method immediately returns an error if
  /// the credential issuer' url cannot be parsed to a DID belonging to one of the trusted issuers. Otherwise an attempt
  /// to verify the credential's signature will be made and an error is returned upon failure.
  #[wasm_bindgen(js_name = verifySignature)]
  pub fn verify_signature(
    credential: &WasmCredential,
    trusted_issuers: &ArrayResolvedDocument,
    options: &WasmVerifierOptions,
  ) -> Result<()> {
    let trusted_issuers: Vec<ResolvedIotaDocument> = trusted_issuers.into_serde().wasm_result()?;
    CredentialValidator::verify_signature(&credential.0, trusted_issuers.as_slice(), &options.0).wasm_result()
  }

  /// Validate that the relationship between the `holder` and the credential subjects is in accordance with
  /// `relationship`.
  pub fn check_subject_holder_relationship(
    credential: &WasmCredential,
    holder: &WasmUrl,
    relationship: WasmSubjectHolderRelationship,
  ) -> Result<()> {
    CredentialValidator::check_subject_holder_relationship(&credential.0, &holder.0, relationship.into()).wasm_result()
  }
}
