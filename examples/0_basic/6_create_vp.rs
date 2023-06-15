// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//!  This example shows how to create a Verifiable Presentation and validate it.
//!  A Verifiable Presentation is the format in which a (collection of) Verifiable Credential(s) gets shared.
//!  It is signed by the subject, to prove control over the Verifiable Credential with a nonce or timestamp.
//!
//! cargo run --example 6_create_vp

use examples::create_did;
use examples::MemStorage;
use identity_iota::core::Object;
use identity_iota::credential::vc_jwt_validation::DecodedJwtCredential;
use identity_iota::credential::DecodedJwtPresentation;
use identity_iota::credential::Jwt;
use identity_iota::credential::JwtPresentation;
use identity_iota::credential::JwtPresentationBuilder;
use identity_iota::credential::JwtPresentationOptions;
use identity_iota::credential::JwtPresentationValidationOptions;
use identity_iota::credential::JwtPresentationValidator;
use identity_iota::did::CoreDID;
use identity_iota::document::verifiable::JwsVerificationOptions;
use identity_iota::storage::JwkDocumentExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::JwsSignatureOptions;
use identity_iota::storage::KeyIdMemstore;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::types::block::address::Address;

use examples::random_stronghold_path;
use examples::API_ENDPOINT;
use identity_iota::core::json;
use identity_iota::core::Duration;
use identity_iota::core::FromJson;
use identity_iota::core::Timestamp;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::CredentialBuilder;
use identity_iota::credential::CredentialValidationOptions;
use identity_iota::credential::CredentialValidator;
use identity_iota::credential::FailFast;
use identity_iota::credential::Subject;
use identity_iota::credential::SubjectHolderRelationship;
use identity_iota::did::DID;
use identity_iota::iota::IotaDocument;
use identity_iota::resolver::Resolver;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // ===========================================================================
  // Step 1: Create identities for the issuer and the holder.
  // ===========================================================================

  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder()
    .with_primary_node(API_ENDPOINT, None)?
    .finish()
    .await?;

  // Create an identity for the issuer with one verification method `key-1`.
  let mut secret_manager_issuer: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password_1")
      .build(random_stronghold_path())?,
  );
  let storage_issuer: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let (_, issuer_document, fragment_issuer): (Address, IotaDocument, String) =
    create_did(&client, &mut secret_manager_issuer, &storage_issuer).await?;

  // Create an identity for the holder, in this case also the subject.
  let mut secret_manager_alice: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password_2")
      .build(random_stronghold_path())?,
  );
  let storage_alice: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let (_, alice_document, fragment_alice): (Address, IotaDocument, String) =
    create_did(&client, &mut secret_manager_alice, &storage_alice).await?;

  // ===========================================================================
  // Step 2: Issuer creates and signs a Verifiable Credential.
  // ===========================================================================

  // Create a credential subject indicating the degree earned by Alice.
  let subject: Subject = Subject::from_json_value(json!({
    "id": alice_document.id().as_str(),
    "name": "Alice",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts",
    },
    "GPA": "4.0",
  }))?;

  // Build credential using subject above and issuer.
  let credential: Credential = CredentialBuilder::default()
    .id(Url::parse("https://example.edu/credentials/3732")?)
    .issuer(Url::parse(issuer_document.id().as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .build()?;

  let credential_jwt: Jwt = issuer_document
    .sign_credential(
      &credential,
      &storage_issuer,
      &fragment_issuer,
      &JwsSignatureOptions::default(),
    )
    .await?;

  // Before sending this credential to the holder the issuer wants to validate that some properties
  // of the credential satisfy their expectations.

  // Validate the credential's signature using the issuer's DID Document, the credential's semantic structure,
  // that the issuance date is not in the future and that the expiration date is not in the past:
  CredentialValidator::new()
    .validate::<_, Object>(
      &credential_jwt,
      &issuer_document,
      &CredentialValidationOptions::default(),
      FailFast::FirstError,
    )
    .unwrap();

  println!("VC successfully validated");

  // ===========================================================================
  // Step 3: Issuer sends the Verifiable Credential to the holder.
  // ===========================================================================
  println!("Sending credential (as JWT) to the holder: {credential:#}");

  // ===========================================================================
  // Step 4: Verifier sends the holder a challenge and requests a signed Verifiable Presentation.
  // ===========================================================================

  // A unique random challenge generated by the requester per presentation can mitigate replay attacks.
  let challenge: &str = "475a7984-1bb5-4c4c-a56f-822bccd46440";

  // The verifier and holder also agree that the signature should have an expiry date
  // 10 minutes from now.
  let expires: Timestamp = Timestamp::now_utc().checked_add(Duration::minutes(10)).unwrap();

  // ===========================================================================
  // Step 5: Holder creates and signs a verifiable presentation from the issued credential.
  // ===========================================================================

  // Create an unsigned Presentation from the previously issued Verifiable Credential.
  let presentation: JwtPresentation =
    JwtPresentationBuilder::new(alice_document.id().to_url().into(), Default::default())
      .credential(credential_jwt)
      .build()?;

  // Sign the verifiable presentation using the holder's verification method
  // and include the requested challenge and expiry timestamp.
  let presentation_jwt: Jwt = alice_document
    .sign_presentation(
      &presentation,
      &storage_alice,
      &fragment_alice,
      &JwsSignatureOptions::default().nonce(challenge.to_owned()),
      &JwtPresentationOptions::default().expiration_date(expires),
    )
    .await?;

  // ===========================================================================
  // Step 6: Holder sends a verifiable presentation to the verifier.
  // ===========================================================================
  println!("Sending presentation (as JWT) to the verifier: {presentation:#}");

  // ===========================================================================
  // Step 7: Verifier receives the Verifiable Presentation and verifies it.
  // ===========================================================================

  // The verifier wants the following requirements to be satisfied:
  // - Signature verification (including checking the requested challenge to mitigate replay attacks)
  // - Presentation validation must fail if credentials expiring within the next 10 hours are encountered
  // - The presentation holder must always be the subject, regardless of the presence of the nonTransferable property
  // - The issuance date must not be in the future.

  let presentation_verifier_options: JwsVerificationOptions =
    JwsVerificationOptions::default().nonce(challenge.to_owned());

  let presentation_validation_options =
    JwtPresentationValidationOptions::default().presentation_verifier_options(presentation_verifier_options);

  let holder_did: CoreDID = JwtPresentationValidator::extract_holder(&presentation_jwt)?;
  let mut resolver: Resolver<IotaDocument> = Resolver::new();
  resolver.attach_iota_handler(client);

  let holder: IotaDocument = resolver.resolve(&holder_did).await?;

  // Validate presentation, Note that this doesn't validate the included credentials.
  let presentation: DecodedJwtPresentation =
    JwtPresentationValidator::new().validate(&presentation_jwt, &holder, &presentation_validation_options)?;

  // Validate the credentials in the presentation.
  let credential_validator = CredentialValidator::new();
  for credential_jwt in presentation.presentation.verifiable_credential.iter() {
    let issuer_did: CoreDID = CredentialValidator::extract_issuer_from_jwt(credential_jwt)?;
    let resolved_issuer_doc: IotaDocument = resolver.resolve(&issuer_did).await?;
    // Validate credential.
    let decoded_credential: DecodedJwtCredential<Object> = credential_validator
      .validate::<_, Object>(
        credential_jwt,
        &resolved_issuer_doc,
        &CredentialValidationOptions::default(),
        FailFast::FirstError,
      )
      .unwrap();
    // Check the presentation holder is the credential subject.
    CredentialValidator::check_subject_holder_relationship(
      &decoded_credential.credential,
      &holder_did.to_url().into(),
      SubjectHolderRelationship::AlwaysSubject,
    )?;
  }

  // Since no errors were thrown by `verify_presentation` we know that the validation was successful.
  println!("VP successfully validated: {:#?}", presentation.presentation);

  // Note that we did not declare a latest allowed issuance date for credentials. This is because we only want to check
  // that the credentials do not have an issuance date in the future which is a default check.
  Ok(())
}
