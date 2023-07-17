// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::common::ImportedDocumentLock;
use crate::credential::WasmDomainLinkageConfiguration;
use crate::credential::WasmJwtCredentialValidationOptions;
use crate::did::IToCoreDocument;
use crate::error::Result;
use crate::error::WasmResult;
use crate::verification::IJwsVerifier;
use crate::verification::WasmJwsVerifier;
use identity_iota::core::Url;
use identity_iota::credential::DomainLinkageValidator;
use wasm_bindgen::prelude::wasm_bindgen;

use super::WasmJwt;

/// A validator for a Domain Linkage Configuration and Credentials.
#[wasm_bindgen(js_name = DomainLinkageValidator)]
pub struct WasmDomainLinkageValidator {
  validator: DomainLinkageValidator<WasmJwsVerifier>,
}

#[wasm_bindgen(js_class = DomainLinkageValidator)]
impl WasmDomainLinkageValidator {
  /// Creates a new `DomainLinkageValidator`. If a `signatureVerifier` is provided it will be used when
  /// verifying decoded JWS signatures, otherwise the default which is only capable of handling the `EdDSA`
  /// algorithm will be used.
  #[wasm_bindgen(constructor)]
  #[allow(non_snake_case)]
  pub fn new(signatureVerifier: Option<IJwsVerifier>) -> WasmDomainLinkageValidator {
    let signature_verifier = WasmJwsVerifier::new(signatureVerifier);
    WasmDomainLinkageValidator {
      validator: DomainLinkageValidator::with_signature_verifier(signature_verifier),
    }
  }

  /// Validates the linkage between a domain and a DID.
  /// `DomainLinkageConfiguration` is validated according to [DID Configuration Resource Verification](https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource-verification).
  ///
  /// Linkage is valid if no error is thrown.
  ///
  /// # Note:
  /// - Only the [JSON Web Token Proof Format](https://identity.foundation/.well-known/resources/did-configuration/#json-web-token-proof-format)
  ///   is supported.
  /// - Only the Credential issued by `issuer` is verified.
  ///
  /// # Errors
  ///
  ///  - Semantic structure of `configuration` is invalid.
  ///  - `configuration` includes multiple credentials issued by `issuer`.
  ///  - Validation of the matched Domain Linkage Credential fails.
  #[wasm_bindgen(js_name = validateLinkage)]
  pub fn validate_linkage(
    &self,
    issuer: &IToCoreDocument,
    configuration: &WasmDomainLinkageConfiguration,
    domain: &str,
    options: &WasmJwtCredentialValidationOptions,
  ) -> Result<()> {
    let domain = Url::parse(domain).wasm_result()?;
    let doc = ImportedDocumentLock::from(issuer);
    let doc_guard = doc.blocking_read();
    self
      .validator
      .validate_linkage(&doc_guard, &configuration.0, &domain, &options.0)
      .wasm_result()
  }

  /// Validates a [Domain Linkage Credential](https://identity.foundation/.well-known/resources/did-configuration/#domain-linkage-credential).
  ///
  /// Error will be thrown in case the validation fails.
  #[wasm_bindgen(js_name = validateCredential)]
  #[allow(non_snake_case)]
  pub fn validate_credential(
    &self,
    issuer: &IToCoreDocument,
    credentialJwt: &WasmJwt,
    domain: &str,
    options: &WasmJwtCredentialValidationOptions,
  ) -> Result<()> {
    let domain = Url::parse(domain).wasm_result()?;
    let doc = ImportedDocumentLock::from(issuer);
    let doc_guard = doc.blocking_read();
    self
      .validator
      .validate_credential(&doc_guard, &credentialJwt.0, &domain, &options.0)
      .wasm_result()
  }
}
