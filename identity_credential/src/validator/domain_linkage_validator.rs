// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::ValidatorDocument;
use crate::credential::Credential;
use crate::credential::DomainLinkageConfiguration;
use crate::validator::errors::DomainLinkageVerificationError;
use crate::validator::errors::DomainLinkageVerificationErrorCause;
use crate::validator::CredentialValidationOptions;
use crate::validator::CredentialValidator;
use crate::validator::FailFast;
use identity_core::common::OneOrMany;
use identity_core::common::Url;
use identity_did::did::CoreDID;
use serde::Serialize;
use std::collections::HashSet;

type DomainLinkageValidationResult = Result<(), DomainLinkageVerificationError>;

/// A verifier for a Domain Linkage Configuration and Credentials.
pub struct DomainLinkageVerifier {}

impl DomainLinkageVerifier {
  /// Verifies the linkage between a domain and a DID.
  /// [`DomainLinkageConfiguration`] is verified according to [DID Configuration Resource Verification](https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource-verification).
  ///
  /// * `issuer`: DID Document of the linked DID. Issuer of the Domain Linkage Credential included
  /// in the Domain Linkage Configuration.
  /// * `configuration`: Domain Linkage Configuration fetched from the domain at "/.well-known/did-configuration.json".
  /// * `domain`: domain, from which the Domain Linkage Configuration has been fetched.
  /// * `validation_options`: Further validation options to be applied on the Domain Linkage Credential.
  ///
  /// # Note:
  /// - Only [Linked Data Proof Format](https://identity.foundation/.well-known/resources/did-configuration/#linked-data-proof-format)
  ///   is supported.
  /// - Only the Credential issued by `issuer` is verified.
  ///
  /// # Errors
  ///  - Semantic structure of `configuration` is invalid.
  ///  - `configuration` includes multiple credentials issued by `issuer`.
  ///  - Verification of the matched Domain Linkage Credential fails.
  pub fn verify_linkage<DOC: ValidatorDocument>(
    issuer: &DOC,
    configuration: &DomainLinkageConfiguration,
    domain: &Url,
    validation_options: &CredentialValidationOptions,
  ) -> DomainLinkageValidationResult {
    configuration
      .check_structure()
      .map_err(|err| DomainLinkageVerificationError {
        cause: DomainLinkageVerificationErrorCause::InvalidStructure,
        source: Some(Box::new(err)),
      })?;

    let mut matched_credentials: Vec<&Credential> = configuration
      .linked_dids()
      .iter()
      .filter(|credential| credential.issuer.url().as_str() == issuer.did_str())
      .collect();

    if matched_credentials.is_empty() {
      return Err(DomainLinkageVerificationError {
        cause: DomainLinkageVerificationErrorCause::CredentialNotFound,
        source: None,
      });
    }

    if matched_credentials.len() == 1 {
      // Unwrap is fine since length is checked.
      let credential = matched_credentials.pop().unwrap();
      return Self::verify_credential(issuer, credential, domain.clone(), validation_options);
    }

    Err(DomainLinkageVerificationError {
      cause: DomainLinkageVerificationErrorCause::InvalidStructure,
      source: None,
    })
  }

  /// Verifies a [Domain Linkage Credential](https://identity.foundation/.well-known/resources/did-configuration/#domain-linkage-credential).
  ///
  /// *`issuer`: issuer of the credential.
  /// *`credential`: domain linkage Credential to be verified.
  /// *`domain`: the domain hosting the credential.
  pub fn verify_credential<T: Serialize, DOC: ValidatorDocument>(
    issuer: &DOC,
    credential: &Credential<T>,
    domain: Url,
    validation_options: &CredentialValidationOptions,
  ) -> DomainLinkageValidationResult {
    credential
      .check_structure()
      .map_err(|err| DomainLinkageVerificationError {
        cause: DomainLinkageVerificationErrorCause::CredentialValidationError,
        source: Some(Box::new(err)),
      })?;

    CredentialValidator::check_issued_on_or_before(credential, credential.issuance_date).map_err(|err| {
      DomainLinkageVerificationError {
        cause: DomainLinkageVerificationErrorCause::CredentialValidationError,
        source: Some(Box::new(err)),
      }
    })?;

    let issuer_did: CoreDID =
      CoreDID::parse(credential.issuer.url().to_string()).map_err(|_err| DomainLinkageVerificationError {
        cause: DomainLinkageVerificationErrorCause::InvalidIssuer,
        source: None,
      })?;

    CredentialValidator::validate(credential, issuer, validation_options, FailFast::AllErrors).map_err(|err| {
      DomainLinkageVerificationError {
        cause: DomainLinkageVerificationErrorCause::CredentialValidationError,
        source: Some(Box::new(err)),
      }
    })?;

    if credential.id.is_some() {
      return Err(DomainLinkageVerificationError {
        cause: DomainLinkageVerificationErrorCause::ImpermissibleIdProperty,
        source: None,
      });
    }

    match &credential.types {
      OneOrMany::Many(types) => {
        if types.len() != 2 {
          Err(DomainLinkageVerificationError {
            cause: DomainLinkageVerificationErrorCause::InvalidTypeProperty,
            source: None,
          })?;
        }
        let type_1 = types.get(0).ok_or(DomainLinkageVerificationError {
          cause: DomainLinkageVerificationErrorCause::InvalidTypeProperty,
          source: None,
        })?;
        let type_2: &String = types.get(1).ok_or(DomainLinkageVerificationError {
          cause: DomainLinkageVerificationErrorCause::InvalidTypeProperty,
          source: None,
        })?;
        let expected_types = HashSet::from([
          Credential::<T>::base_type(),
          DomainLinkageConfiguration::domain_linkage_type(),
        ]);
        let types = HashSet::from([type_1.as_str(), type_2.as_str()]);
        if !types.eq(&expected_types) {
          Err(DomainLinkageVerificationError {
            cause: DomainLinkageVerificationErrorCause::InvalidTypeProperty,
            source: None,
          })?;
        }
      }
      OneOrMany::One(_) => {
        Err(DomainLinkageVerificationError {
          cause: DomainLinkageVerificationErrorCause::InvalidTypeProperty,
          source: None,
        })?;
      }
    };

    match credential.expiration_date {
      None => {
        return Err(DomainLinkageVerificationError {
          cause: DomainLinkageVerificationErrorCause::MissingExpirationDate,
          source: None,
        });
      }
      Some(expiration_date) => {
        CredentialValidator::check_expires_on_or_after(credential, expiration_date).map_err(|err| {
          DomainLinkageVerificationError {
            cause: DomainLinkageVerificationErrorCause::CredentialValidationError,
            source: Some(Box::new(err)),
          }
        })?;
      }
    }

    match &credential.credential_subject {
      OneOrMany::One(credential_subject) => {
        match &credential_subject.id {
          None => {
            return Err(DomainLinkageVerificationError {
              cause: DomainLinkageVerificationErrorCause::MissingSubjectId,
              source: None,
            });
          }
          Some(id) => match CoreDID::parse(id.to_string()) {
            Ok(subject_did) => {
              if issuer_did != subject_did {
                return Err(DomainLinkageVerificationError {
                  cause: DomainLinkageVerificationErrorCause::IssuerSubjectMismatch,
                  source: None,
                });
              }
            }
            Err(_) => {
              return Err(DomainLinkageVerificationError {
                cause: DomainLinkageVerificationErrorCause::InvalidSubjectId,
                source: None,
              });
            }
          },
        }

        let origin: &str = credential_subject
          .properties
          .get("origin")
          .ok_or(DomainLinkageVerificationError {
            cause: DomainLinkageVerificationErrorCause::InvalidSubjectOrigin,
            source: None,
          })?
          .as_str()
          .ok_or(DomainLinkageVerificationError {
            cause: DomainLinkageVerificationErrorCause::InvalidSubjectOrigin,
            source: None,
          })?;
        let origin_url = Url::parse(origin)
          .map_err(|_err| Url::parse("https://".to_owned() + origin))
          .map_err(|_err| DomainLinkageVerificationError {
            cause: DomainLinkageVerificationErrorCause::InvalidSubjectOrigin,
            source: None,
          })?;

        if origin_url.origin() != domain.origin() {
          return Err(DomainLinkageVerificationError {
            cause: DomainLinkageVerificationErrorCause::OriginMismatch,
            source: None,
          });
        }
      }
      OneOrMany::Many(_) => {
        return Err(DomainLinkageVerificationError {
          cause: DomainLinkageVerificationErrorCause::MultipleCredentialSubjects,
          source: None,
        });
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::credential::Credential;
  use crate::credential::DomainLinkageConfiguration;
  use crate::validator::domain_linkage_validator::DomainLinkageVerifier;
  use crate::validator::CredentialValidationOptions;
  use identity_core::common::Url;
  use identity_core::convert::FromJson;
  use identity_did::document::CoreDocument;

  #[test]
  pub(crate) fn test() {
    let configuration_string: &str = include_str!("../../tests/fixtures/did_configuration/config1.json");
    let mut configuration: DomainLinkageConfiguration =
      DomainLinkageConfiguration::from_json(configuration_string).unwrap();
    let credential: &mut Credential = configuration.linked_dids_mut().get_mut(0).unwrap();

    let configuration_string: &str = include_str!("../../tests/fixtures/did_configuration/issuer-did-document.json");
    let document: CoreDocument = CoreDocument::from_json(configuration_string).unwrap();
    let domain: Url = Url::parse("https://example.com").unwrap();
    DomainLinkageVerifier::verify_credential(&document, credential, domain, &CredentialValidationOptions::default())
      .unwrap();
  }
}
