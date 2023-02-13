// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Result;
use crate::error::WasmResult;
use crate::resolver::RustSupportedDocument;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::CredentialValidationOptions;
use identity_iota::credential::DomainLinkageConfiguration;
use identity_iota::credential::DomainLinkageValidator;
use proc_typescript::typescript;
use wasm_bindgen::prelude::wasm_bindgen;

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
  pub fn validate_linkage(options: IValidateLinkage) -> Result<()> {
    let IValidateLinkageHelper {
      issuer,
      configuration,
      domain,
      validation_options,
    } = options.into_serde::<IValidateLinkageHelper>().wasm_result()?;

    match issuer {
      RustSupportedDocument::Core(doc) => {
        DomainLinkageValidator::validate_linkage(&doc, &configuration, &domain, &validation_options).wasm_result()?;
      }
      RustSupportedDocument::Iota(doc) => {
        DomainLinkageValidator::validate_linkage(&doc, &configuration, &domain, &validation_options).wasm_result()?;
      }
    }
    Ok(())
  }

  /// Validates a [Domain Linkage Credential](https://identity.foundation/.well-known/resources/did-configuration/#domain-linkage-credential).
  /// Error will be thrown in case the validation fails.
  #[wasm_bindgen(js_name = validateCredential)]
  pub fn validate_credential(options: IValidateCredential) -> Result<()> {
    let IValidateCredentialHelper {
      issuer,
      credential,
      domain,
      validation_options,
    } = options.into_serde::<IValidateCredentialHelper>().wasm_result()?;

    match issuer {
      RustSupportedDocument::Core(doc) => {
        DomainLinkageValidator::validate_credential(&doc, &credential, &domain, &validation_options).wasm_result()?;
      }
      RustSupportedDocument::Iota(doc) => {
        DomainLinkageValidator::validate_credential(&doc, &credential, &domain, &validation_options).wasm_result()?;
      }
    }
    Ok(())
  }
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IValidateLinkage")]
  pub type IValidateLinkage;
}

/// Fields for validating a domain linkage.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[typescript(name = "IValidateLinkage", readonly, optional)]
struct IValidateLinkageHelper {
  /// DID Document of the linked DID. Issuer of the Domain Linkage Credential included
  /// in the Domain Linkage Configuration.
  #[typescript(optional = false, type = "CoreDocument | IotaDocument")]
  issuer: RustSupportedDocument,
  /// Domain Linkage Configuration fetched from the domain at "/.well-known/did-configuration.json".
  #[typescript(optional = false, type = "DomainLinkageConfiguration")]
  configuration: DomainLinkageConfiguration,
  /// Domain from which the Domain Linkage Configuration has been fetched.
  #[typescript(optional = false, type = "string")]
  domain: Url,
  /// Further validation options to be applied on the Domain Linkage Credential.
  #[typescript(optional = false, name = "validationOptions", type = "CredentialValidationOptions")]
  validation_options: CredentialValidationOptions,
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "IValidateCredential")]
  pub type IValidateCredential;
}

/// Fields for validating a Domain Linkage Credential.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[typescript(name = "IValidateCredential", readonly, optional)]
struct IValidateCredentialHelper {
  /// DID Document of the linked DID. Issuer of the Domain Linkage Credential included
  /// in the Domain Linkage Configuration.
  #[typescript(optional = false, type = "CoreDocument | IotaDocument")]
  issuer: RustSupportedDocument,
  /// Domain Linkage Configuration fetched from the domain at "/.well-known/did-configuration.json".
  #[typescript(optional = false, type = "Credential")]
  credential: Credential,
  /// Domain from which the Domain Linkage Configuration has been fetched.
  #[typescript(optional = false, type = "string")]
  domain: Url,
  /// Further validation options to be applied on the Domain Linkage Credential.
  #[typescript(optional = false, name = "validationOptions", type = "CredentialValidationOptions")]
  validation_options: CredentialValidationOptions,
}
