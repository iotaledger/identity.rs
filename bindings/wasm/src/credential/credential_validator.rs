// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_iota::core::Object;
use identity_iota::core::Url;
use identity_iota::credential::CredentialValidator;
use identity_iota::credential::StatusCheck;
use identity_iota::credential::ValidationError;
use identity_iota::did::CoreDID;
use wasm_bindgen::prelude::*;

use crate::common::ImportedDocumentLock;
use crate::common::ImportedDocumentReadGuard;
use crate::common::WasmTimestamp;
use crate::credential::validation_options::WasmFailFast;
use crate::credential::validation_options::WasmStatusCheck;
use crate::did::ArrayIToCoreDocument;
use crate::did::IToCoreDocument;
use crate::did::WasmCoreDID;
use crate::did::WasmVerifierOptions;
use crate::error::Result;
use crate::error::WasmResult;

use super::WasmCredential;
use super::WasmCredentialValidationOptions;
use super::WasmSubjectHolderRelationship;

#[wasm_bindgen(js_name = CredentialValidator, inspectable)]
pub struct WasmCredentialValidator;

#[wasm_bindgen(js_class = CredentialValidator)]
impl WasmCredentialValidator {
  /// Validates a `Credential`.
  ///
  /// The following properties are validated according to `options`:
  /// - the issuer's signature,
  /// - the expiration date,
  /// - the issuance date,
  /// - the semantic structure.
  ///
  /// ### Warning
  /// The lack of an error returned from this method is in of itself not enough to conclude that the credential can be
  /// trusted. This section contains more information on additional checks that should be carried out before and after
  /// calling this method.
  ///
  /// #### The state of the issuer's DID Document
  /// The caller must ensure that `issuer` represents an up-to-date DID Document. The convenience method
  /// `Resolver::resolveCredentialIssuer` can help extract the latest available state of the issuer's DID Document.
  ///
  /// #### Properties that are not validated
  ///  There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
  /// `credentialStatus`, `type`, `credentialSchema`, `refreshService`, **and more**.
  /// These should be manually checked after validation, according to your requirements.
  ///
  /// ### Errors
  /// An error is returned whenever a validated condition is not satisfied.
  #[wasm_bindgen]
  pub fn validate(
    credential: &WasmCredential,
    issuer: &IToCoreDocument,
    options: &WasmCredentialValidationOptions,
    fail_fast: WasmFailFast,
  ) -> Result<()> {
    let issuer_lock = ImportedDocumentLock::from(issuer);
    let issuer_guard = issuer_lock.blocking_read();
    //let issuer: RustSupportedDocument = issuer.into_serde::<RustSupportedDocument>().wasm_result()?;
    CredentialValidator::validate(&credential.0, &issuer_guard, &options.0, fail_fast.into()).wasm_result()
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
  pub fn check_expires_on_or_after(credential: &WasmCredential, timestamp: &WasmTimestamp) -> Result<()> {
    CredentialValidator::check_expires_on_or_after(&credential.0, timestamp.0).wasm_result()
  }

  /// Validate that the credential is issued on or before the specified timestamp.
  #[wasm_bindgen(js_name = checkIssuedOnOrBefore)]
  pub fn check_issued_on_or_before(credential: &WasmCredential, timestamp: &WasmTimestamp) -> Result<()> {
    CredentialValidator::check_issued_on_or_before(&credential.0, timestamp.0).wasm_result()
  }

  /// Verify the signature using the DID Document of a trusted issuer.
  ///
  /// # Warning
  /// The caller must ensure that the DID Documents of the trusted issuers are up-to-date.
  /// ### Errors
  /// This method immediately returns an error if
  /// the credential issuer' url cannot be parsed to a DID belonging to one of the trusted issuers. Otherwise an attempt
  /// to verify the credential's signature will be made and an error is returned upon failure.
  #[wasm_bindgen(js_name = verifySignature)]
  #[allow(non_snake_case)]
  pub fn verify_signature(
    credential: &WasmCredential,
    trustedIssuers: &ArrayIToCoreDocument,
    options: &WasmVerifierOptions,
  ) -> Result<()> {
    let issuer_locks: Vec<ImportedDocumentLock> = trustedIssuers.into();
    let trusted_issuers: Vec<ImportedDocumentReadGuard<'_>> =
      issuer_locks.iter().map(ImportedDocumentLock::blocking_read).collect();
    CredentialValidator::verify_signature(&credential.0, &trusted_issuers, &options.0).wasm_result()
  }

  /// Validate that the relationship between the `holder` and the credential subjects is in accordance with
  /// `relationship`. The `holder` parameter is expected to be the URL of the holder.
  #[wasm_bindgen(js_name = checkSubjectHolderRelationship)]
  pub fn check_subject_holder_relationship(
    credential: &WasmCredential,
    holder: &str,
    relationship: WasmSubjectHolderRelationship,
  ) -> Result<()> {
    let holder: Url = Url::parse(holder).wasm_result()?;
    CredentialValidator::check_subject_holder_relationship(&credential.0, &holder, relationship.into()).wasm_result()
  }

  /// Checks whether the credential status has been revoked.
  ///
  /// Only supports `BitmapRevocation2022`.
  #[wasm_bindgen(js_name = checkStatus)]
  #[allow(non_snake_case)]
  pub fn check_status(
    credential: &WasmCredential,
    trustedIssuers: &ArrayIToCoreDocument,
    statusCheck: WasmStatusCheck,
  ) -> Result<()> {
    let issuer_locks: Vec<ImportedDocumentLock> = trustedIssuers.into();
    let trusted_issuers: Vec<ImportedDocumentReadGuard<'_>> =
      issuer_locks.iter().map(ImportedDocumentLock::blocking_read).collect();
    let status_check: StatusCheck = StatusCheck::from(statusCheck);
    CredentialValidator::check_status(&credential.0, &trusted_issuers, status_check).wasm_result()
  }

  /// Utility for extracting the issuer field of a `Credential` as a DID.
  ///
  /// ### Errors
  ///
  /// Fails if the issuer field is not a valid DID.
  #[wasm_bindgen(js_name = extractIssuer)]
  pub fn extract_issuer(credential: &WasmCredential) -> Result<WasmCoreDID> {
    CredentialValidator::extract_issuer::<CoreDID, Object>(&credential.0)
      .map(WasmCoreDID::from)
      .wasm_result()
  }
}
