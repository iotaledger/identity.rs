// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
#[cfg(test)]
mod tests {
  use identity_core::common::Duration;
  use identity_core::common::Object;
  use identity_core::common::Timestamp;
  use identity_core::common::Url;
  use identity_core::convert::FromJson;
  use identity_core::crypto::KeyPair;
  use identity_core::crypto::KeyType;
  use identity_core::crypto::ProofOptions;
  use identity_core::json;
  use identity_core::utils::BaseEncoding;
  use identity_did::document::CoreDocument;
  use identity_did::verifiable::VerifierOptions;
  use identity_did::verification::MethodScope;
  use identity_did::verification::VerificationMethod;

  use identity_resolver::Resolver;

  use crate::StardustDocument;
  use crate::StardustVerificationMethod;

  use identity_credential::credential::Credential;
  use identity_credential::credential::CredentialBuilder;
  use identity_credential::credential::Subject;
  use identity_credential::presentation::Presentation;
  use identity_credential::validator::CredentialValidationOptions;
  use identity_credential::validator::CredentialValidator;
  use identity_credential::validator::FailFast;
  use identity_credential::validator::PresentationValidationOptions;
  use identity_credential::validator::PresentationValidator;
  use identity_credential::validator::SubjectHolderRelationship;
  use identity_credential::validator::ValidatorDocument;
  use identity_credential::validator::AbstractValidatorDocument;
  use identity_did::did::CoreDID;
  use identity_did::did::DID;

  fn generate_stardust_document(keypair: &KeyPair) -> StardustDocument {
    let mut document: StardustDocument = StardustDocument::new_with_id(
      "did:stardust:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
        .parse()
        .unwrap(),
    );
    let method: StardustVerificationMethod =
      StardustVerificationMethod::new(document.id().clone(), keypair.type_(), keypair.public(), "issuerKey").unwrap();
    document.insert_method(method, MethodScope::VerificationMethod).unwrap();
    document
  }

  fn generate_core_document() -> (CoreDocument, KeyPair) {
    let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
    let did: CoreDID = CoreDID::parse(&format!(
      "did:example:{}",
      BaseEncoding::encode_base58(keypair.public())
    ))
    .unwrap();
    let document: CoreDocument = CoreDocument::builder(Object::new())
      .id(did.clone())
      .verification_method(VerificationMethod::new(did, KeyType::Ed25519, keypair.public(), "#root").unwrap())
      .build()
      .unwrap();
    (document, keypair)
  }

  fn generate_credential(issuer: &str, subject: &str) -> Credential {
    let credential_subject: Subject = Subject::from_json_value(json!({
      "id": subject,
      "name": "Alice",
      "degree": {
        "type": "BachelorDegree",
        "name": "Bachelor of Science and Arts",
      },
      "GPA": "4.0",
    }))
    .unwrap();

    // Build credential using subject above and issuer.
    CredentialBuilder::default()
      .id(Url::parse("https://example.edu/credentials/3732").unwrap())
      .issuer(Url::parse(issuer).unwrap())
      .type_("UniversityDegreeCredential")
      .subject(credential_subject)
      .issuance_date(Timestamp::now_utc())
      .expiration_date(Timestamp::now_utc().checked_add(Duration::days(1)).unwrap())
      .build()
      .unwrap()
  }

  fn generate_presentation(holder: &str, credentials: Vec<Credential>) -> Presentation {
    let mut builder = Presentation::builder(Object::new())
      .id(Url::parse("https://example.org/credentials/3732").unwrap())
      .holder(Url::parse(holder).unwrap());
    for credential in credentials {
      builder = builder.credential(credential);
    }
    builder.build().unwrap()
  }

  // Convenience struct for setting up tests.
  struct MixedTestSetup {
    // Issuer of credential_stardust.
    issuer_stardust_doc: StardustDocument,
    issuer_stardust_key: KeyPair,
    credential_stardust: Credential,
    // Issuer of credential_core.
    issuer_core_doc: CoreDocument,
    issuer_core_key: KeyPair,
    credential_core: Credential,
    // Subject of both credentials.
    subject_doc: CoreDocument,
    subject_key: KeyPair,
  }

  impl MixedTestSetup {
    // Creates DID Documents and unsigned credentials.
    fn new() -> Self {
      let (issuer_stardust_doc, issuer_stardust_key) = {
        let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
        (generate_stardust_document(&keypair), keypair)
      };
      let (subject_doc, subject_key) = generate_core_document();
      let credential_stardust = generate_credential(issuer_stardust_doc.id().as_str(), subject_doc.id().as_str());

      let (issuer_core_doc, issuer_core_key) = generate_core_document();
      let credential_core = generate_credential(issuer_core_doc.id().as_str(), subject_doc.id().as_str());

      Self {
        issuer_stardust_doc,
        issuer_stardust_key,
        subject_doc,
        subject_key,
        credential_stardust,
        issuer_core_doc,
        issuer_core_key,
        credential_core,
      }
    }

    // Creates DID Documents with signed credentials.
    fn new_with_signed_credentials() -> Self {
      let mut setup = Self::new();
      let MixedTestSetup {
        ref issuer_stardust_doc,
        ref issuer_stardust_key,
        ref mut credential_stardust,
        ref issuer_core_doc,
        ref issuer_core_key,
        ref mut credential_core,
        ..
      } = setup;

      issuer_stardust_doc
        .signer(issuer_stardust_key.private())
        .options(ProofOptions::default())
        .method(issuer_stardust_doc.methods().next().unwrap().id())
        .sign(credential_stardust)
        .unwrap();

      issuer_core_doc
        .signer(issuer_core_key.private())
        .options(ProofOptions::default())
        .method(issuer_core_doc.methods().next().unwrap().id())
        .sign(credential_core)
        .unwrap();
      setup
    }
  }

  #[tokio::test]
  async fn test_resolver_verify_presentation_mixed() {
    let MixedTestSetup {
      issuer_stardust_doc,
      credential_stardust,
      issuer_core_doc,
      credential_core,
      subject_doc,
      subject_key,
      ..
    } = MixedTestSetup::new_with_signed_credentials();

    // Subject signs the presentation.
    let mut presentation = generate_presentation(
      subject_doc.id().as_str(),
      [credential_stardust, credential_core].to_vec(),
    );
    let challenge: String = "475a7984-1bb5-4c4c-a56f-822bccd46441".to_owned();
    subject_doc
      .signer(subject_key.private())
      .options(ProofOptions::new().challenge(challenge.clone()))
      .method(subject_doc.methods().next().unwrap().id())
      .sign(&mut presentation)
      .unwrap();

    // VALID: resolver supports presentations with issuers from different DID Methods.
    let resolver: Resolver = Resolver::new();
    assert!(resolver
      .verify_presentation(
        &presentation,
        &PresentationValidationOptions::new()
          .presentation_verifier_options(VerifierOptions::new().challenge(challenge))
          .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject),
        FailFast::FirstError,
        Some(&subject_doc),
        Some(&[issuer_stardust_doc.as_validator(), issuer_core_doc.as_validator()])
      )
      .await
      .is_ok());
  }

  #[test]
  fn test_validate_presentation_mixed() {
    let MixedTestSetup {
      issuer_stardust_doc,
      credential_stardust,
      issuer_core_doc,
      credential_core,
      subject_doc,
      subject_key,
      ..
    } = MixedTestSetup::new_with_signed_credentials();

    // Subject signs the presentation.
    let mut presentation = generate_presentation(
      subject_doc.id().as_str(),
      [credential_stardust, credential_core].to_vec(),
    );
    let challenge: String = "475a7984-1bb5-4c4c-a56f-822bccd46440".to_owned();
    subject_doc
      .signer(subject_key.private())
      .options(ProofOptions::new().challenge(challenge.clone()))
      .method(subject_doc.methods().next().unwrap().id())
      .sign(&mut presentation)
      .unwrap();

    // Validate presentation.
    let presentation_validation_options = PresentationValidationOptions::new()
      .shared_validation_options(
        CredentialValidationOptions::new()
          .earliest_expiry_date(Timestamp::now_utc().checked_add(Duration::days(1)).unwrap())
          .latest_issuance_date(Timestamp::now_utc()),
      )
      .presentation_verifier_options(VerifierOptions::new().challenge(challenge))
      .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject);

    // VALID: presentations with issuers from different DID Methods are supported.
    assert!(PresentationValidator::validate(
      &presentation,
      &subject_doc,
      &[issuer_stardust_doc.as_validator(), issuer_core_doc.as_validator()],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_ok());

    let issuer_iota_box = AbstractValidatorDocument::from(issuer_stardust_doc.clone());
    let issuer_core_box = AbstractValidatorDocument::from(issuer_core_doc.clone());
    assert!(PresentationValidator::validate(
      &presentation,
      &subject_doc,
      &[issuer_iota_box, issuer_core_box],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_ok());

    // INVALID: wrong holder fails.
    assert!(PresentationValidator::validate(
      &presentation,
      &issuer_stardust_doc,
      &[issuer_stardust_doc.as_validator(), issuer_core_doc.as_validator()],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_err());
    assert!(PresentationValidator::validate(
      &presentation,
      &issuer_core_doc,
      &[issuer_stardust_doc.as_validator(), issuer_core_doc.as_validator()],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_err());

    // INVALID: excluding the IOTA DID Document issuer fails.
    assert!(PresentationValidator::validate(
      &presentation,
      &subject_doc,
      &[issuer_core_doc.as_validator()],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_err());

    // INVALID: excluding the core DID Document issuer fails.
    assert!(PresentationValidator::validate(
      &presentation,
      &subject_doc,
      &[&issuer_stardust_doc],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_err());

    // INVALID: using the wrong core DID Document fails.
    assert!(PresentationValidator::validate(
      &presentation,
      &subject_doc,
      &[issuer_stardust_doc.as_validator(), subject_doc.as_validator()],
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_err());

    // INVALID: excluding all issuers fails.
    let empty_issuers: &[&dyn ValidatorDocument] = &[];
    assert!(PresentationValidator::validate(
      &presentation,
      &subject_doc,
      empty_issuers,
      &presentation_validation_options,
      FailFast::FirstError,
    )
    .is_err());
  }

  #[test]
  fn test_validate_credential_mixed() {
    let MixedTestSetup {
      issuer_stardust_doc,
      credential_stardust,
      issuer_core_doc,
      credential_core,
      subject_doc,
      ..
    } = MixedTestSetup::new_with_signed_credentials();
    let options = CredentialValidationOptions::new()
      .earliest_expiry_date(Timestamp::now_utc().checked_add(Duration::days(1)).unwrap())
      .latest_issuance_date(Timestamp::now_utc());

    // VALID: credential validation works for issuers with different DID Methods.
    assert!(CredentialValidator::validate(
      &credential_stardust,
      &issuer_stardust_doc,
      &options,
      FailFast::FirstError
    )
    .is_ok());
    assert!(CredentialValidator::validate(&credential_core, &issuer_core_doc, &options, FailFast::FirstError).is_ok());

    // INVALID: wrong issuer fails.
    assert!(
      CredentialValidator::validate(&credential_stardust, &issuer_core_doc, &options, FailFast::FirstError).is_err()
    );
    assert!(CredentialValidator::validate(&credential_stardust, &subject_doc, &options, FailFast::FirstError).is_err());
    assert!(
      CredentialValidator::validate(&credential_core, &issuer_stardust_doc, &options, FailFast::FirstError).is_err()
    );
    assert!(CredentialValidator::validate(&credential_core, &subject_doc, &options, FailFast::FirstError).is_err());
  }
}
