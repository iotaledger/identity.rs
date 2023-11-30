// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::credential::Credential;
use crate::credential::Jwt;
use crate::domain_linkage::DomainLinkageConfiguration;
use crate::domain_linkage::DomainLinkageValidationError;
use crate::domain_linkage::DomainLinkageValidationErrorCause;
use crate::validator::FailFast;
use crate::validator::JwtCredentialValidationOptions;
use crate::validator::JwtCredentialValidator;
use identity_core::common::OneOrMany;
use identity_core::common::Url;
use identity_did::CoreDID;
use identity_document::document::CoreDocument;
use identity_verification::jws::JwsVerifier;

use crate::validator::DecodedJwtCredential;

use super::DomainLinkageValidationResult;
use crate::utils::url_only_includes_origin;

/// A validator for a Domain Linkage Configuration and Credentials.

pub struct JwtDomainLinkageValidator<V: JwsVerifier> {
  validator: JwtCredentialValidator<V>,
}

impl<V: JwsVerifier> JwtDomainLinkageValidator<V> {
  /// Create a new [`JwtDomainLinkageValidator`] that delegates cryptographic signature verification to the given
  /// `signature_verifier`.
  pub fn with_signature_verifier(signature_verifier: V) -> Self {
    Self {
      validator: JwtCredentialValidator::with_signature_verifier(signature_verifier),
    }
  }

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
  /// - Only the [JSON Web Token Proof Format](https://identity.foundation/.well-known/resources/did-configuration/#json-web-token-proof-format)
  /// is supported.
  /// - Only the Credential issued by `issuer` is verified.
  ///
  /// # Errors
  ///  - Semantic structure of `configuration` is invalid.
  ///  - `configuration` includes multiple credentials issued by `issuer`.
  ///  - Validation of the matched Domain Linkage Credential fails.
  pub fn validate_linkage<DOC: AsRef<CoreDocument>>(
    &self,
    issuer: &DOC,
    configuration: &DomainLinkageConfiguration,
    domain: &Url,
    validation_options: &JwtCredentialValidationOptions,
  ) -> DomainLinkageValidationResult {
    let issuers: Vec<CoreDID> = configuration.issuers().map_err(|err| DomainLinkageValidationError {
      cause: DomainLinkageValidationErrorCause::InvalidJwt,
      source: Some(err.into()),
    })?;

    // Multiple credentials for the same issuer are an error.
    if issuers.iter().filter(|iss| *iss == issuer.as_ref().id()).count() > 1 {
      return Err(DomainLinkageValidationError {
        cause: DomainLinkageValidationErrorCause::InvalidStructure,
        source: None,
      });
    };

    // Find the index of the issuer in the JWT credentials if present.
    let (jwt_index, _): (usize, _) = issuers
      .iter()
      .enumerate()
      .find(|(_index, iss)| *iss == issuer.as_ref().id())
      .ok_or_else(|| DomainLinkageValidationError {
        cause: DomainLinkageValidationErrorCause::InvalidIssuer,
        source: None,
      })?;

    // Validate the credential at the corresponding index.
    let credential: &Jwt = configuration
      .linked_dids()
      .get(jwt_index)
      .ok_or_else(|| DomainLinkageValidationError {
        cause: DomainLinkageValidationErrorCause::InvalidIssuer,
        source: None,
      })?;

    self.validate_credential(issuer, credential, domain, validation_options)
  }

  /// Validates a [Domain Linkage Credential](https://identity.foundation/.well-known/resources/did-configuration/#domain-linkage-credential).
  ///
  /// *`issuer`: issuer of the credential.
  /// *`credential`: domain linkage Credential to be verified.
  /// *`domain`: the domain hosting the credential.
  pub fn validate_credential<DOC: AsRef<CoreDocument>>(
    &self,
    issuer: &DOC,
    credential: &Jwt,
    domain: &Url,
    validation_options: &JwtCredentialValidationOptions,
  ) -> DomainLinkageValidationResult {
    let decoded_credential: DecodedJwtCredential = self
      .validator
      .validate(credential, issuer, validation_options, FailFast::AllErrors)
      .map_err(|err| DomainLinkageValidationError {
        cause: DomainLinkageValidationErrorCause::CredentialValidationError,
        source: Some(Box::new(err)),
      })?;

    let credential: &Credential = &decoded_credential.credential;

    let issuer_did: CoreDID =
      CoreDID::parse(credential.issuer.url().as_str()).map_err(|err| DomainLinkageValidationError {
        cause: DomainLinkageValidationErrorCause::InvalidIssuer,
        source: Some(Box::new(err)),
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
      if !url_only_includes_origin(&origin_url) {
        return Err(DomainLinkageValidationError {
          cause: DomainLinkageValidationErrorCause::InvalidSubjectOrigin,
          source: None,
        });
      }
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
  use crate::credential::Jws;
  use crate::credential::Jwt;
  use crate::domain_linkage::DomainLinkageConfiguration;
  use crate::domain_linkage::DomainLinkageCredentialBuilder;
  use crate::domain_linkage::DomainLinkageValidationErrorCause;
  use crate::domain_linkage::DomainLinkageValidationResult;
  use crate::domain_linkage::JwtDomainLinkageValidator;
  use crate::validator::test_utils::generate_jwk_document_with_keys;
  use crate::validator::JwtCredentialValidationOptions;

  use crypto::signatures::ed25519::SecretKey;
  use identity_core::common::Duration;
  use identity_core::common::Object;
  use identity_core::common::OneOrMany;
  use identity_core::common::OrderedSet;
  use identity_core::common::Timestamp;
  use identity_core::common::Url;
  use identity_did::CoreDID;
  use identity_document::document::CoreDocument;
  use identity_eddsa_verifier::EdDSAJwsVerifier;
  use identity_verification::jws::CharSet;
  use identity_verification::jws::CompactJwsEncoder;
  use identity_verification::jws::CompactJwsEncodingOptions;
  use identity_verification::jws::JwsAlgorithm;
  use identity_verification::jws::JwsHeader;
  use identity_verification::MethodData;
  use identity_verification::VerificationMethod;
  use once_cell::sync::Lazy;

  static JWT_DOMAIN_LINKAGE_VALIDATOR_ED25519: Lazy<JwtDomainLinkageValidator<EdDSAJwsVerifier>> =
    Lazy::new(|| JwtDomainLinkageValidator::with_signature_verifier(EdDSAJwsVerifier::default()));

  #[test]
  pub(crate) fn test_valid_credential() {
    let (document, secret_key, fragment) = generate_jwk_document_with_keys();
    let credential: Credential = create_domain_linkage_credential(document.id());
    let jwt: Jwt = sign_credential_jwt(&credential, &document, &fragment, &secret_key);

    let validation_result: DomainLinkageValidationResult = JWT_DOMAIN_LINKAGE_VALIDATOR_ED25519.validate_credential(
      &document,
      &jwt,
      &url_foo(),
      &JwtCredentialValidationOptions::default(),
    );

    assert!(validation_result.is_ok());
  }

  #[test]
  pub(crate) fn test_invalid_credential_signature() {
    let (document, _secret_key, fragment) = generate_jwk_document_with_keys();
    let credential: Credential = create_domain_linkage_credential(document.id());
    let other_secret_key: SecretKey = SecretKey::generate().unwrap();
    // Sign with `other_secret_key` to produce an invalid signature.
    let jwt: Jwt = sign_credential_jwt(&credential, &document, &fragment, &other_secret_key);

    let validation_result: DomainLinkageValidationResult = JWT_DOMAIN_LINKAGE_VALIDATOR_ED25519.validate_credential(
      &document,
      &jwt,
      &url_foo(),
      &JwtCredentialValidationOptions::default(),
    );
    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::CredentialValidationError
    ));
  }

  #[test]
  pub(crate) fn test_invalid_id_property() {
    let (document, secret_key, fragment) = generate_jwk_document_with_keys();
    let mut credential: Credential = create_domain_linkage_credential(document.id());
    credential.id = Some(Url::parse("http://random.credential.id").unwrap());
    let jwt: Jwt = sign_credential_jwt(&credential, &document, &fragment, &secret_key);

    let validation_result: DomainLinkageValidationResult = JWT_DOMAIN_LINKAGE_VALIDATOR_ED25519.validate_credential(
      &document,
      &jwt,
      &url_foo(),
      &JwtCredentialValidationOptions::default(),
    );

    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::ImpermissibleIdProperty
    ));
  }

  #[test]
  pub(crate) fn test_domain_linkage_type_missing() {
    let (document, secret_key, fragment) = generate_jwk_document_with_keys();
    let mut credential: Credential = create_domain_linkage_credential(document.id());
    credential.types = OneOrMany::One(Credential::<Object>::base_type().to_owned());
    let jwt: Jwt = sign_credential_jwt(&credential, &document, &fragment, &secret_key);

    let validation_result: DomainLinkageValidationResult = JWT_DOMAIN_LINKAGE_VALIDATOR_ED25519.validate_credential(
      &document,
      &jwt,
      &url_foo(),
      &JwtCredentialValidationOptions::default(),
    );

    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::InvalidTypeProperty
    ));
  }

  #[test]
  pub(crate) fn test_extra_type() {
    let (document, secret_key, fragment) = generate_jwk_document_with_keys();
    let mut credential: Credential = create_domain_linkage_credential(document.id());
    credential.types = OneOrMany::Many(vec![
      Credential::<Object>::base_type().to_owned(),
      DomainLinkageConfiguration::domain_linkage_type().to_owned(),
      "not-allowed-type".to_owned(),
    ]);
    let jwt: Jwt = sign_credential_jwt(&credential, &document, &fragment, &secret_key);

    let validation_result: DomainLinkageValidationResult = JWT_DOMAIN_LINKAGE_VALIDATOR_ED25519.validate_credential(
      &document,
      &jwt,
      &url_foo(),
      &JwtCredentialValidationOptions::default(),
    );

    assert!(validation_result.is_ok());
  }

  #[test]
  pub(crate) fn test_origin_mismatch() {
    let (document, secret_key, fragment) = generate_jwk_document_with_keys();
    let mut credential: Credential = create_domain_linkage_credential(document.id());

    let mut properties: Object = Object::new();
    properties.insert("origin".into(), "http://www.example-1.com".into());
    if let OneOrMany::One(ref mut subject) = credential.credential_subject {
      subject.properties = properties;
    }
    let jwt: Jwt = sign_credential_jwt(&credential, &document, &fragment, &secret_key);

    let validation_result: DomainLinkageValidationResult = JWT_DOMAIN_LINKAGE_VALIDATOR_ED25519.validate_credential(
      &document,
      &jwt,
      &url_foo(),
      &JwtCredentialValidationOptions::default(),
    );

    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::OriginMismatch
    ));
  }

  #[test]
  pub(crate) fn test_empty_origin() {
    let (document, secret_key, fragment) = generate_jwk_document_with_keys();
    let mut credential: Credential = create_domain_linkage_credential(document.id());

    let properties: Object = Object::new();
    if let OneOrMany::One(ref mut subject) = credential.credential_subject {
      subject.properties = properties;
    }
    let jwt: Jwt = sign_credential_jwt(&credential, &document, &fragment, &secret_key);

    let validation_result: DomainLinkageValidationResult = JWT_DOMAIN_LINKAGE_VALIDATOR_ED25519.validate_credential(
      &document,
      &jwt,
      &url_foo(),
      &JwtCredentialValidationOptions::default(),
    );

    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::InvalidSubjectOrigin
    ));
  }

  #[test]
  pub(crate) fn test_origin_without_scheme() {
    let (document, secret_key, fragment) = generate_jwk_document_with_keys();
    let mut credential: Credential = create_domain_linkage_credential(document.id());

    let mut properties: Object = Object::new();
    properties.insert("origin".into(), "foo.example.com".into());
    if let OneOrMany::One(ref mut subject) = credential.credential_subject {
      subject.properties = properties;
    }
    let jwt: Jwt = sign_credential_jwt(&credential, &document, &fragment, &secret_key);

    let validation_result: DomainLinkageValidationResult = JWT_DOMAIN_LINKAGE_VALIDATOR_ED25519.validate_credential(
      &document,
      &jwt,
      &url_foo(),
      &JwtCredentialValidationOptions::default(),
    );

    assert!(validation_result.is_ok());
  }

  #[test]
  pub(crate) fn test_no_subject_id() {
    let (document, secret_key, fragment) = generate_jwk_document_with_keys();
    let mut credential: Credential = create_domain_linkage_credential(document.id());

    if let OneOrMany::One(ref mut subject) = credential.credential_subject {
      subject.id = None;
    }
    let jwt: Jwt = sign_credential_jwt(&credential, &document, &fragment, &secret_key);

    let validation_result: DomainLinkageValidationResult = JWT_DOMAIN_LINKAGE_VALIDATOR_ED25519.validate_credential(
      &document,
      &jwt,
      &url_foo(),
      &JwtCredentialValidationOptions::default(),
    );

    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::MissingSubjectId
    ));
  }

  #[test]
  pub(crate) fn test_invalid_subject_id() {
    let (document, secret_key, fragment) = generate_jwk_document_with_keys();
    let mut credential: Credential = create_domain_linkage_credential(document.id());

    if let OneOrMany::One(ref mut subject) = credential.credential_subject {
      subject.id = Some(Url::parse("http://invalid.did").unwrap());
    }

    let jwt: Jwt = sign_credential_jwt(&credential, &document, &fragment, &secret_key);

    let validation_result: DomainLinkageValidationResult = JWT_DOMAIN_LINKAGE_VALIDATOR_ED25519.validate_credential(
      &document,
      &jwt,
      &url_foo(),
      &JwtCredentialValidationOptions::default(),
    );

    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::InvalidSubjectId
    ));
  }

  #[test]
  pub(crate) fn test_issuer_subject_mismatch() {
    let (document, secret_key, fragment) = generate_jwk_document_with_keys();
    let mut credential: Credential = create_domain_linkage_credential(document.id());

    if let OneOrMany::One(ref mut subject) = credential.credential_subject {
      subject.id = Some(Url::parse("did:abc:xyz").unwrap());
    }
    let jwt: Jwt = sign_credential_jwt(&credential, &document, &fragment, &secret_key);

    let validation_result: DomainLinkageValidationResult = JWT_DOMAIN_LINKAGE_VALIDATOR_ED25519.validate_credential(
      &document,
      &jwt,
      &url_foo(),
      &JwtCredentialValidationOptions::default(),
    );

    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::IssuerSubjectMismatch
    ));
  }

  #[test]
  pub(crate) fn test_multiple_credentials_for_same_did() {
    let (document, secret_key, fragment) = generate_jwk_document_with_keys();
    let credential: Credential = create_domain_linkage_credential(document.id());
    let jwt: Jwt = sign_credential_jwt(&credential, &document, &fragment, &secret_key);

    let configuration: DomainLinkageConfiguration = DomainLinkageConfiguration::new(vec![jwt.clone(), jwt]);

    let validation_result: DomainLinkageValidationResult = JWT_DOMAIN_LINKAGE_VALIDATOR_ED25519.validate_linkage(
      &document,
      &configuration,
      &url_foo(),
      &JwtCredentialValidationOptions::default(),
    );
    assert!(matches!(
      validation_result.unwrap_err().cause,
      DomainLinkageValidationErrorCause::InvalidStructure
    ));
  }

  #[test]
  pub(crate) fn test_valid_configuration() {
    let (document, secret_key, fragment) = generate_jwk_document_with_keys();
    let credential: Credential = create_domain_linkage_credential(document.id());
    let jwt: Jwt = sign_credential_jwt(&credential, &document, &fragment, &secret_key);

    let configuration: DomainLinkageConfiguration = DomainLinkageConfiguration::new(vec![jwt]);
    let validation_result: DomainLinkageValidationResult = JWT_DOMAIN_LINKAGE_VALIDATOR_ED25519.validate_linkage(
      &document,
      &configuration,
      &url_foo(),
      &JwtCredentialValidationOptions::default(),
    );

    assert!(validation_result.is_ok());
  }

  fn url_foo() -> Url {
    Url::parse("https://foo.example.com").unwrap()
  }

  fn create_domain_linkage_credential(did: &CoreDID) -> Credential {
    let domain: Url = url_foo();

    let mut domains: OrderedSet<Url> = OrderedSet::new();
    domains.append(domain.clone());

    let credential: Credential = DomainLinkageCredentialBuilder::new()
      .issuer(did.clone())
      .origin(domain)
      .issuance_date(Timestamp::now_utc())
      .expiration_date(Timestamp::now_utc().checked_add(Duration::days(365)).unwrap())
      .build()
      .unwrap();
    credential
  }

  fn sign_credential_jwt(
    credential: &Credential,
    document: &CoreDocument,
    fragment: &str,
    secret_key: &SecretKey,
  ) -> Jwt {
    let payload: String = credential.serialize_jwt(None).unwrap();
    Jwt::new(sign_bytes(document, fragment, payload.as_ref(), secret_key).into())
  }

  fn sign_bytes(document: &CoreDocument, fragment: &str, payload: &[u8], secret_key: &SecretKey) -> Jws {
    let method: &VerificationMethod = document.resolve_method(fragment, None).unwrap();
    let MethodData::PublicKeyJwk(ref jwk) = method.data() else {
      panic!("not a jwk");
    };
    let alg: JwsAlgorithm = jwk.alg().unwrap_or("").parse().unwrap();

    let header: JwsHeader = {
      let mut header = JwsHeader::new();
      header.set_alg(alg);
      header.set_kid(method.id().to_string());
      header
    };

    let encoding_options: CompactJwsEncodingOptions = CompactJwsEncodingOptions::NonDetached {
      charset_requirements: CharSet::Default,
    };

    let jws_encoder: CompactJwsEncoder<'_> =
      CompactJwsEncoder::new_with_options(payload, &header, encoding_options).unwrap();

    let signature: [u8; 64] = secret_key.sign(jws_encoder.signing_input()).to_bytes();

    Jws::new(jws_encoder.into_jws(&signature))
  }
}
