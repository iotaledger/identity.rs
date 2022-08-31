// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use identity_core::common::Duration;
use identity_core::common::Object;
use identity_core::common::Timestamp;
use identity_core::common::Url;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::crypto::KeyPair;
use identity_core::crypto::KeyType;
use identity_core::crypto::ProofOptions;
use identity_core::json;
use identity_core::utils::BaseEncoding;
use identity_credential::credential::Credential;
use identity_credential::credential::CredentialBuilder;
use identity_credential::credential::Subject;
use identity_credential::presentation::Presentation;
use identity_credential::validator::AbstractThreadSafeValidatorDocument;
use identity_credential::validator::FailFast;
use identity_credential::validator::PresentationValidationOptions;
use identity_credential::validator::SubjectHolderRelationship;
use identity_credential::validator::ValidatorDocument;
use identity_did::did::CoreDID;
use identity_did::did::DID;
use identity_did::document::CoreDocument;
use identity_did::verifiable::VerifierOptions;
use identity_did::verification::MethodScope;
use identity_did::verification::VerificationMethod;

use crate::StardustDID;
use crate::StardustDocument;
use crate::StardustVerificationMethod;

const TEST_METHOD_NAME_0: &'static str = "foo";
const TEST_METHOD_NAME_1: &'static str = "bar";

fn generate_stardust_document(keypair: &KeyPair) -> StardustDocument {
  let mut document: StardustDocument = StardustDocument::new_with_id(
    format!(
      "did:{}:0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
      StardustDID::METHOD
    )
    .as_str()
    .parse()
    .unwrap(),
  );
  let method: StardustVerificationMethod =
    StardustVerificationMethod::new(document.id().clone(), keypair.type_(), keypair.public(), "issuerKey").unwrap();
  document.insert_method(method, MethodScope::VerificationMethod).unwrap();
  document
}

fn generate_core_document(method_name: String, keypair: KeyPair) -> (CoreDocument, KeyPair) {
  let did: CoreDID = CoreDID::parse(&format!(
    "did:{}:{}",
    method_name,
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
  // Issuer of credential_stardust (did method: StardustDID::METHOD = "stardust").
  issuer_stardust_doc: StardustDocument,
  issuer_stardust_key: KeyPair,
  credential_stardust: Credential,
  // Issuer of credential_core (did method: TestDID::<1>() = "test1").
  issuer_core_doc: CoreDocument,
  issuer_core_key: KeyPair,
  credential_core: Credential,
  // Subject of both credentials (did method: TestDID::<0>() = "test0").
  subject_doc: CoreDocument,
  subject_key: KeyPair,
}

impl MixedTestSetup {
  // Creates DID Documents and unsigned credentials.
  fn new() -> Self {
    let (issuer_stardust_doc, issuer_stardust_key) = {
      let hex_encoded: &str = "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60";
      let private_key = BaseEncoding::decode(hex_encoded, identity_core::utils::Base::Base16Lower).unwrap();
      //println!("private key: {:?}", &private_key.as_slice());
      //let keypair: KeyPair = KeyPair::new(KeyType::Ed25519).unwrap();
      let keypair: KeyPair = KeyPair::try_from_private_key_bytes(KeyType::Ed25519, &private_key).unwrap();
      (generate_stardust_document(&keypair), keypair)
    };
    let subject_private_key_hex_encoded: &str = "4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb";
    let subject_private_key =
      BaseEncoding::decode(subject_private_key_hex_encoded, identity_core::utils::Base::Base16Lower).unwrap();
    let subject_key_pair: KeyPair =
      KeyPair::try_from_private_key_bytes(KeyType::Ed25519, &subject_private_key).unwrap();

    let (subject_doc, subject_key) = generate_core_document(TEST_METHOD_NAME_0.to_string(), subject_key_pair);

    let credential_stardust = generate_credential(issuer_stardust_doc.id().as_str(), subject_doc.id().as_str());

    let issuer_private_key_hex: &str = "c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7";
    let issuer_private_key =
      BaseEncoding::decode(issuer_private_key_hex, identity_core::utils::Base::Base16Lower).unwrap();
    let issuer_core_key_pair: KeyPair =
      KeyPair::try_from_private_key_bytes(KeyType::Ed25519, &issuer_private_key).unwrap();
    let (issuer_core_doc, issuer_core_key) =
      generate_core_document(TEST_METHOD_NAME_1.to_string(), issuer_core_key_pair);
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

/*
async fn test_generic_resolver_verify_presentation<DOC: ValidatorDocument + Send + Sync + 'static>(
  signed_presentation: &Presentation,
  challenge: String,
  resolver: Resolver<DOC>,
) {
  let holder_doc = resolver.resolve_presentation_holder(signed_presentation).await.unwrap();
  let issuer_docs = resolver
    .resolve_presentation_issuers(signed_presentation)
    .await
    .unwrap();

  // check that verification works regardless of whether we first resolve and then pass holder/issuers to the method or
  // if resolution of missing documents is done internally.
  for pass_holder_as_arg in [true, false] {
    for pass_issuers_as_arg in [true, false] {
      let holder: Option<&DOC> = pass_holder_as_arg.then_some(&holder_doc);
      let issuers: Option<&[DOC]> = pass_issuers_as_arg.then_some(&issuer_docs);
      assert!(resolver
        .verify_presentation(
          signed_presentation,
          &PresentationValidationOptions::new()
            .presentation_verifier_options(VerifierOptions::new().challenge(challenge.clone()))
            .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject),
          FailFast::FirstError,
          holder,
          issuers
        )
        .await
        .is_ok());
    }
  }
}

*/

#[tokio::test]
/// Tests verifying a presentation under the following circumstances:
/// The subjects's did method: test0
/// issuer_stardust's did method: stardust
/// issuer_core's did method: test1
/// Verify the presentation with both Resolver<CoreDocument> and the dynamic resolver (Resolver<Box<dyn
/// ValidatorDocument>>).
async fn test_verify_presentation() {
  let MixedTestSetup {
    issuer_stardust_doc,
    credential_stardust,
    issuer_core_doc,
    credential_core,
    subject_doc,
    subject_key,
    ..
  } = MixedTestSetup::new_with_signed_credentials();

  println!("{}", issuer_core_doc.to_json_pretty().unwrap());
  panic!();
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

  /*
  // Check that verification works with the resolver converting all resolved documents to CoreDocument.
  //let resolver_core: Resolver<CoreDocument> = setup_resolver::<CoreDocument>(foo_client.clone(), bar_client.clone());
  // Check that verification works with the resolver converting all resolved documents to the boxed trait object Box<dyn
  // ValidatorDocument>.
  let resolver_dynamic: Resolver =
    setup_resolver::<AbstractThreadSafeValidatorDocument>(foo_client.clone(), bar_client);

  test_generic_resolver_verify_presentation(&presentation, challenge.clone(), resolver_core).await;
  test_generic_resolver_verify_presentation(&presentation, challenge.clone(), resolver_dynamic).await;

  */
}
/*
fn setup_resolver<DOC>(foo_client: FooClient, bar_client: Arc<BarClient>) -> Resolver<DOC>
where
  DOC: ValidatorDocument + From<CoreDocument> + From<StardustDocument> + 'static + Send + Sync,
{
  let mut resolver: Resolver<DOC> = Resolver::new();

  resolver.attach_handler(StardustDID::METHOD.to_string(), move |did: StardustDID| {
    let foo = foo_client.clone();
    async move { foo.resolve(&did).await }
  });

  let bar_client_clone = bar_client.clone();
  resolver.attach_handler(TEST_METHOD_NAME_0.to_string(), move |did: CoreDID| {
    let bar = bar_client_clone.clone();
    async move { bar.resolve(&did).await }
  });

  let bar_client_clone = bar_client.clone();
  resolver.attach_handler(TEST_METHOD_NAME_1.to_string(), move |did: CoreDID| {
    let bar = bar_client_clone.clone();
    async move { bar.resolve(&did).await }
  });

  resolver
}
*/

/*
#[tokio::test]
//TODO: Avoid loads of code repetition.
async fn verify_presentation_dynamic_resolver_core_documents() {
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

  let resolver: Resolver = Resolver::new();

  let issuers: Vec<&dyn ValidatorDocument> = vec![issuer_stardust_doc.as_validator(), issuer_core_doc.as_validator()];

  assert!(resolver
    .verify_presentation(
      &presentation,
      &PresentationValidationOptions::new()
        .presentation_verifier_options(VerifierOptions::new().challenge(challenge.clone()))
        .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject),
      FailFast::FirstError,
      Some(&subject_doc),
      Some(&issuers)
    )
    .await
    .is_ok());

  let resolver: Resolver<CoreDocument> = Resolver::new();

  assert!(resolver
    .verify_presentation(
      &presentation,
      &PresentationValidationOptions::new()
        .presentation_verifier_options(VerifierOptions::new().challenge(challenge.clone()))
        .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject),
      FailFast::FirstError,
      Some(&subject_doc),
      Some(&issuers)
    )
    .await
    .is_ok());

  let issuers: Vec<CoreDocument> = vec![issuer_stardust_doc.into(), issuer_core_doc.into()];

  assert!(resolver
    .verify_presentation(
      &presentation,
      &PresentationValidationOptions::new()
        .presentation_verifier_options(VerifierOptions::new().challenge(challenge.clone()))
        .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject),
      FailFast::FirstError,
      Some(&subject_doc),
      Some(&issuers)
    )
    .await
    .is_ok());

  assert!(resolver
    .verify_presentation(
      &presentation,
      &PresentationValidationOptions::new()
        .presentation_verifier_options(VerifierOptions::new().challenge(challenge.clone()))
        .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject),
      FailFast::FirstError,
      Some(&subject_doc.as_validator()),
      Some(&issuers)
    )
    .await
    .is_ok());

  let resolver: Resolver = Resolver::new();

  assert!(resolver
    .verify_presentation(
      &presentation,
      &PresentationValidationOptions::new()
        .presentation_verifier_options(VerifierOptions::new().challenge(challenge.clone()))
        .subject_holder_relationship(SubjectHolderRelationship::AlwaysSubject),
      FailFast::FirstError,
      Some(&subject_doc.as_validator()),
      Some(&issuers)
    )
    .await
    .is_ok());
}
*/
