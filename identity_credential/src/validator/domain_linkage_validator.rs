// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::credential::Credential;
use crate::credential::DomainLinkageConfiguration;
use crate::validator::errors::DomainLinkageValidationError;
use crate::validator::errors::DomainLinkageValidationErrorCause;
use crate::validator::CredentialValidationOptions;
use crate::validator::CredentialValidator;
use crate::validator::FailFast;
use identity_core::common::OneOrMany;
use identity_core::common::Url;
use identity_did::CoreDID;
use identity_did::DID;
use identity_document::document::CoreDocument;
use serde::Serialize;

type DomainLinkageValidationResult = Result<(), DomainLinkageValidationError>;

/// A validator for a Domain Linkage Configuration and Credentials.
pub struct DomainLinkageValidator {}

impl DomainLinkageValidator {
  /// Validates the linkage between a domain and a DID.
  /// [`DomainLinkageConfiguration`] is validated according to [DID Configuration Resource Verification](https://identity.foundation/.well-known/resources/did-configuration/#did-configuration-resource-verification).
  ///
  /// * `issuer`: DID Document of the linked DID. Issuer of the Domain Linkage Credential included
  /// in the Domain Linkage Configuration.
  /// * `configuration`: Domain Linkage Configuration fetched from the domain at "/.well-known/did-configuration.json".
  /// * `domain`: domain from which the Domain Linkage Configuration has been fetched.
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
  ///  - Validation of the matched Domain Linkage Credential fails.
  pub fn validate_linkage<DOC: AsRef<CoreDocument>>(
    issuer: &DOC,
    configuration: &DomainLinkageConfiguration,
    domain: &Url,
    validation_options: &CredentialValidationOptions,
  ) -> DomainLinkageValidationResult {
    let mut matched_credentials = configuration
      .linked_dids()
      .iter()
      .filter(|credential| credential.issuer.url().as_str() == CoreDocument::id(issuer.as_ref()).as_str());

    match matched_credentials.next() {
      None => Err(DomainLinkageValidationError {
        cause: DomainLinkageValidationErrorCause::InvalidStructure,
        source: None,
      }),
      Some(credential) => {
        if matched_credentials.next().is_some() {
          Err(DomainLinkageValidationError {
            cause: DomainLinkageValidationErrorCause::InvalidStructure,
            source: None,
          })
        } else {
          Self::validate_credential(issuer, credential, domain.clone(), validation_options)
        }
      }
    }
  }

  /// Validates a [Domain Linkage Credential](https://identity.foundation/.well-known/resources/did-configuration/#domain-linkage-credential).
  ///
  /// *`issuer`: issuer of the credential.
  /// *`credential`: domain linkage Credential to be verified.
  /// *`domain`: the domain hosting the credential.
  pub fn validate_credential<T: Serialize, DOC: AsRef<CoreDocument>>(
    issuer: &DOC,
    credential: &Credential<T>,
    domain: Url,
    validation_options: &CredentialValidationOptions,
  ) -> DomainLinkageValidationResult {
    let issuer_did: CoreDID =
      CoreDID::parse(credential.issuer.url().as_str()).map_err(|err| DomainLinkageValidationError {
        cause: DomainLinkageValidationErrorCause::InvalidIssuer,
        source: Some(Box::new(err)),
      })?;

    CredentialValidator::validate(credential, issuer, validation_options, FailFast::AllErrors).map_err(|err| {
      DomainLinkageValidationError {
        cause: DomainLinkageValidationErrorCause::CredentialValidationError,
        source: Some(Box::new(err)),
      }
    })?;

    if credential.id.is_some() {
      return Err(DomainLinkageValidationError {
        cause: DomainLinkageValidationErrorCause::ImpermissibleIdProperty,
        source: None,
      });
    }

    // Validate type.
    if !credential
      .types
      .iter()
      .any(|type_| type_ == DomainLinkageConfiguration::domain_linkage_type())
    {
      return Err(DomainLinkageValidationError {
        cause: DomainLinkageValidationErrorCause::InvalidTypeProperty,
        source: None,
      });
    }

    // Extract credential subject.
    let OneOrMany::One(ref credential_subject) = credential.credential_subject else {
      return Err(DomainLinkageValidationError {
        cause: DomainLinkageValidationErrorCause::MultipleCredentialSubjects,
        source: None,
      });
    };

    // Validate credential subject.
    {
      let subject_id = credential_subject.id.as_deref().ok_or(DomainLinkageValidationError {
        cause: DomainLinkageValidationErrorCause::MissingSubjectId,
        source: None,
      })?;
      let subject_did = CoreDID::parse(subject_id.as_str()).map_err(|_| DomainLinkageValidationError {
        cause: DomainLinkageValidationErrorCause::InvalidSubjectId,
        source: None,
      })?;
      if issuer_did != subject_did {
        return Err(DomainLinkageValidationError {
          cause: DomainLinkageValidationErrorCause::IssuerSubjectMismatch,
          source: None,
        });
      }
    }

    // Extract and validate origin.
    {
      let origin: &str = credential_subject
        .properties
        .get("origin")
        .and_then(|value| value.as_str())
        .ok_or(DomainLinkageValidationError {
          cause: DomainLinkageValidationErrorCause::InvalidSubjectOrigin,
          source: None,
        })?;
      let origin_url: Url = match Url::parse(origin) {
        Ok(url) => Ok(url),
        Err(identity_core::Error::InvalidUrl(url::ParseError::RelativeUrlWithoutBase)) => {
          Url::parse("https://".to_owned() + origin).map_err(|err| DomainLinkageValidationError {
            cause: DomainLinkageValidationErrorCause::InvalidSubjectOrigin,
            source: Some(Box::new(err)),
          })
        }
        Err(other_error) => Err(DomainLinkageValidationError {
          cause: DomainLinkageValidationErrorCause::InvalidSubjectOrigin,
          source: Some(Box::new(other_error)),
        }),
      }?;
      if origin_url.origin() != domain.origin() {
        return Err(DomainLinkageValidationError {
          cause: DomainLinkageValidationErrorCause::OriginMismatch,
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
  use crate::credential::DomainLinkageCredentialBuilder;
  use crate::credential::Issuer;
  use crate::credential::Subject;
  use crate::validator::domain_linkage_validator::DomainLinkageValidationResult;
  use crate::validator::domain_linkage_validator::DomainLinkageValidator;
  use crate::validator::errors::DomainLinkageValidationErrorCause;
  use crate::validator::test_utils::generate_document_with_keys;
  use crate::validator::CredentialValidationOptions;

  use identity_core::common::Duration;
  use identity_core::common::Object;
  use identity_core::common::OneOrMany;
  use identity_core::common::OrderedSet;
  use identity_core::common::Timestamp;
  use identity_core::common::Url;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::ProofOptions;
  use identity_did::DID;
  use identity_document::document::CoreDocument;

  #[test]
  pub(crate) fn test_valid_credential() {
    let (mut credential, document, keypair) = create_valid_credential();
    sign_credential(&mut credential, &document, keypair);
    let validation_result: DomainLinkageValidationResult = DomainLinkageValidator::validate_credential(
      &document,
      &credential,
      url_foo(),
      &CredentialValidationOptions::default(),
    );
    assert!(validation_result.is_ok());
  }

  #[test]
  pub(crate) fn test_invalid_credential_signature() {
    let (mut credential, document, keypair) = create_valid_credential();
    sign_credential(&mut credential, &document, keypair);
    credential.expiration_date = Some(Timestamp::now_utc().checked_add(Duration::days(10)).unwrap());
    let validation_result: DomainLinkageValidationResult = DomainLinkageValidator::validate_credential(
      &document,
      &credential,
      url_foo(),
      &CredentialValidationOptions::default(),
    );
    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::CredentialValidationError
    ));
  }

  #[test]
  pub(crate) fn test_invalid_id_property() {
    let (mut credential, document, keypair) = create_valid_credential();
    credential.id = Some(Url::parse("http://random.credential.id").unwrap());
    sign_credential(&mut credential, &document, keypair);
    let validation_result: DomainLinkageValidationResult = DomainLinkageValidator::validate_credential(
      &document,
      &credential,
      url_foo(),
      &CredentialValidationOptions::default(),
    );
    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::ImpermissibleIdProperty
    ));
  }

  #[test]
  pub(crate) fn test_domain_linkage_type_missing() {
    let (mut credential, document, keypair) = create_valid_credential();
    credential.types = OneOrMany::One(Credential::<Object>::base_type().to_owned());
    sign_credential(&mut credential, &document, keypair);
    let validation_result: DomainLinkageValidationResult = DomainLinkageValidator::validate_credential(
      &document,
      &credential,
      url_foo(),
      &CredentialValidationOptions::default(),
    );
    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::InvalidTypeProperty
    ));
  }

  #[test]
  pub(crate) fn test_extra_type() {
    let (mut credential, document, keypair) = create_valid_credential();
    credential.types = OneOrMany::Many(vec![
      Credential::<Object>::base_type().to_owned(),
      DomainLinkageConfiguration::domain_linkage_type().to_owned(),
      "not-allowed-type".to_owned(),
    ]);
    sign_credential(&mut credential, &document, keypair);
    let validation_result: DomainLinkageValidationResult = DomainLinkageValidator::validate_credential(
      &document,
      &credential,
      url_foo(),
      &CredentialValidationOptions::default(),
    );
    assert!(validation_result.is_ok());
  }

  #[test]
  pub(crate) fn test_origin_mismatch() {
    let (mut credential, document, keypair) = create_valid_credential();
    let mut properties: Object = Object::new();
    properties.insert("origin".into(), "http://www.example-1.com".into());
    if let OneOrMany::One(ref mut subject) = credential.credential_subject {
      subject.properties = properties;
    }
    sign_credential(&mut credential, &document, keypair);
    let validation_result: DomainLinkageValidationResult = DomainLinkageValidator::validate_credential(
      &document,
      &credential,
      url_foo(),
      &CredentialValidationOptions::default(),
    );
    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::OriginMismatch
    ));
  }

  #[test]
  pub(crate) fn test_empty_origin() {
    let (mut credential, document, keypair) = create_valid_credential();
    let properties: Object = Object::new();
    if let OneOrMany::One(ref mut subject) = credential.credential_subject {
      subject.properties = properties;
    }
    sign_credential(&mut credential, &document, keypair);
    let validation_result: DomainLinkageValidationResult = DomainLinkageValidator::validate_credential(
      &document,
      &credential,
      url_foo(),
      &CredentialValidationOptions::default(),
    );
    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::InvalidSubjectOrigin
    ));
  }

  #[test]
  pub(crate) fn test_origin_without_scheme() {
    let (mut credential, document, keypair) = create_valid_credential();
    let mut properties: Object = Object::new();
    properties.insert("origin".into(), "foo.example.com".into());
    if let OneOrMany::One(ref mut subject) = credential.credential_subject {
      subject.properties = properties;
    }
    sign_credential(&mut credential, &document, keypair);
    let validation_result: DomainLinkageValidationResult = DomainLinkageValidator::validate_credential(
      &document,
      &credential,
      url_foo(),
      &CredentialValidationOptions::default(),
    );
    assert!(validation_result.is_ok());
  }

  #[test]
  pub(crate) fn test_multiple_subjects() {
    let (mut credential, document, keypair) = create_valid_credential();
    let mut subjects: Vec<Subject> = credential.credential_subject.clone().to_vec();
    subjects.push(subjects.get(0).unwrap().clone());
    let subjects = OneOrMany::Many(subjects);
    credential.credential_subject = subjects;
    sign_credential(&mut credential, &document, keypair);
    let validation_result: DomainLinkageValidationResult = DomainLinkageValidator::validate_credential(
      &document,
      &credential,
      url_foo(),
      &CredentialValidationOptions::default(),
    );
    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::MultipleCredentialSubjects
    ));
  }

  #[test]
  pub(crate) fn test_no_subject_id() {
    let (mut credential, document, keypair) = create_valid_credential();
    if let OneOrMany::One(ref mut subject) = credential.credential_subject {
      subject.id = None;
    }
    sign_credential(&mut credential, &document, keypair);
    let validation_result: DomainLinkageValidationResult = DomainLinkageValidator::validate_credential(
      &document,
      &credential,
      url_foo(),
      &CredentialValidationOptions::default(),
    );
    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::MissingSubjectId
    ));
  }

  #[test]
  pub(crate) fn test_invalid_subject_id() {
    let (mut credential, document, keypair) = create_valid_credential();
    if let OneOrMany::One(ref mut subject) = credential.credential_subject {
      subject.id = Some(Url::parse("http://invalid.did").unwrap());
    }
    sign_credential(&mut credential, &document, keypair);
    let validation_result: DomainLinkageValidationResult = DomainLinkageValidator::validate_credential(
      &document,
      &credential,
      url_foo(),
      &CredentialValidationOptions::default(),
    );
    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::InvalidSubjectId
    ));
  }

  #[test]
  pub(crate) fn test_issuer_subject_mismatch() {
    let (mut credential, document, keypair) = create_valid_credential();
    if let OneOrMany::One(ref mut subject) = credential.credential_subject {
      subject.id = Some(Url::parse("did:abc:xyz").unwrap());
    }
    sign_credential(&mut credential, &document, keypair);
    let validation_result: DomainLinkageValidationResult = DomainLinkageValidator::validate_credential(
      &document,
      &credential,
      url_foo(),
      &CredentialValidationOptions::default(),
    );
    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::IssuerSubjectMismatch
    ));
  }

  #[test]
  pub(crate) fn test_multiple_credentials_for_same_did() {
    let (mut credential, document, keypair) = create_valid_credential();
    sign_credential(&mut credential, &document, keypair);
    let configuration: DomainLinkageConfiguration =
      DomainLinkageConfiguration::new(vec![credential.clone(), credential]);
    let validation_result: DomainLinkageValidationResult = DomainLinkageValidator::validate_linkage(
      &document,
      &configuration,
      &url_foo(),
      &CredentialValidationOptions::default(),
    );
    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::InvalidStructure
    ));
  }

  #[test]
  pub(crate) fn test_valid_configuration() {
    let (mut credential, document, keypair) = create_valid_credential();
    sign_credential(&mut credential, &document, keypair);
    let configuration: DomainLinkageConfiguration = DomainLinkageConfiguration::new(vec![credential]);
    let validation_result: DomainLinkageValidationResult = DomainLinkageValidator::validate_linkage(
      &document,
      &configuration,
      &url_foo(),
      &CredentialValidationOptions::default(),
    );
    assert!(validation_result.is_ok());
  }

  fn url_foo() -> Url {
    Url::parse("https://foo.example.com").unwrap()
  }

  fn create_valid_credential() -> (Credential, CoreDocument, KeyPair) {
    let (doc, keypair) = generate_document_with_keys();
    let domain_1: Url = url_foo();

    let mut domains: OrderedSet<Url> = OrderedSet::new();
    domains.append(domain_1.clone());

    let credential: Credential = DomainLinkageCredentialBuilder::new()
      .issuer(Issuer::Url(doc.id().to_url().into()))
      .origin(domain_1)
      .issuance_date(Timestamp::now_utc())
      .expiration_date(Timestamp::now_utc().checked_add(Duration::days(365)).unwrap())
      .build()
      .unwrap();
    (credential, doc, keypair)
  }

  fn sign_credential(credential: &mut Credential, document: &CoreDocument, keypair: KeyPair) {
    document
      .signer(keypair.private())
      .options(ProofOptions::default())
      .method(document.methods(None).get(0).unwrap().id())
      .sign(credential)
      .unwrap();
  }
}
