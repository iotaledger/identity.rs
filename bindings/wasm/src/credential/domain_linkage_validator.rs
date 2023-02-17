// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::ImportedDocumentLock;
use crate::credential::WasmDomainLinkageConfiguration;
use crate::did::IToCoreDocument;
use crate::error::Result;
use crate::error::WasmResult;
use identity_iota::core::Url;
use identity_iota::credential::DomainLinkageValidator;
use wasm_bindgen::prelude::wasm_bindgen;

use super::WasmCredential;
use super::WasmCredentialValidationOptions;

/// A validator for a Domain Linkage Configuration and Credentials.
#[wasm_bindgen(js_name = DomainLinkageValidator)]
pub struct WasmDomainLinkageValidator;

#[wasm_bindgen(js_class = DomainLinkageValidator)]
impl WasmDomainLinkageValidator {
  /// Validates the linkage between a domain and a DID.
  /// [`DomainLinkageConfiguration`] is validated according to [DID Configuration Resource Verification](https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource-verification).
  ///
  /// Linkage is valid if no error is thrown.
  ///
  /// # Note:
  /// - Only [Linked Data Proof Format](https://identity.foundation/.well-known/resources/did-configuration/#linked-data-proof-format)
  ///   is supported.
  /// - Only the Credential issued by `issuer` is verified.
  ///
  /// # Errors
  ///  - Semantic structure of `configuration` is invalid.
  ///  - `configuration` includes multiple credentials issued by `issuer`.
  ///  - Validation of the matched Domain Linkage Credential fails.
  #[wasm_bindgen(js_name = validateLinkage)]
  pub fn validate_linkage(
    issuer: &IToCoreDocument,
    configuration: &WasmDomainLinkageConfiguration,
    domain: &str,
    options: &WasmCredentialValidationOptions,
  ) -> Result<()> {
    let domain = Url::parse(domain).wasm_result()?;
    let doc = ImportedDocumentLock::from(issuer);
    let doc_guard = doc.blocking_read();
    DomainLinkageValidator::validate_linkage(&doc_guard, &configuration.0, &domain, &options.0).wasm_result()
  }

  /// Validates a [Domain Linkage Credential](https://identity.foundation/.well-known/resources/did-configuration/#domain-linkage-credential).
  /// Error will be thrown in case the validation fails.
  #[wasm_bindgen(js_name = validateCredential)]
  pub fn validate_credential(
    issuer: &IToCoreDocument,
    credential: &WasmCredential,
    domain: &str,
    options: &WasmCredentialValidationOptions,
  ) -> Result<()> {
    let domain = Url::parse(domain).wasm_result()?;
    let doc = ImportedDocumentLock::from(issuer);
    let doc_guard = doc.blocking_read();
    DomainLinkageValidator::validate_credential(&doc_guard, &credential.0, &domain, &options.0).wasm_result()
  }
}
