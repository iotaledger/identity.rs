use std::{collections::HashMap, fs::File, path::Path};

use examples::{create_did, random_stronghold_path, MemStorage, API_ENDPOINT};
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::{core::{Duration, FromJson, Object, Timestamp, Url}, credential::{Credential, CredentialBuilder, DecodedJwtCredential, DecodedJwtPresentation, FailFast, Jwt, JwtCredentialValidationOptions, JwtCredentialValidator, JwtCredentialValidatorUtils, JwtPresentationOptions, JwtPresentationValidationOptions, JwtPresentationValidator, JwtPresentationValidatorUtils, Presentation, PresentationBuilder, Subject, SubjectHolderRelationship}, did::{CoreDID, DID}, document::{verifiable::JwsVerificationOptions, CoreDocument}, iota::IotaDocument, resolver::Resolver, storage::{JwkMemStore, JwsSignatureOptions, KeyIdMemstore}, verification::{jws::JwsAlgorithm, MethodScope}};
use identity_iota::storage::JwkDocumentExt;
use iota_sdk::{client::{secret::{stronghold::StrongholdSecretManager, SecretManager}, Client, Password}, types::block::address::Address};
use reqwest::ClientBuilder;
use serde_json::json;

pub fn write_to_file(doc: &CoreDocument, path: Option<&str>) -> anyhow::Result<()> {
  let path = Path::new(path.unwrap_or_else(|| "did.json"));
  let file = File::create(path)?;
  serde_json::to_writer_pretty(file, doc)?;
  Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

  let did_url: &str = "https://localhost:4443/.well-known/did.json";
  let path_did_file: &str = "C:/Projects/did-web-server/.well-known/did.json";

  // Create a new client to make HTTPS requests.
  let client= ClientBuilder::new()
  .danger_accept_invalid_certs(true)
  .build()?;

  // Create a new Web DID document.
  let mut issuer_document: CoreDocument = CoreDocument::new_from_url(did_url)?;

  // Insert a new Ed25519 verification method in the DID document.
  let storage_issuer: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let fragment_issuer = issuer_document
    .generate_method(
      &storage_issuer,
      JwkMemStore::ED25519_KEY_TYPE,
      JwsAlgorithm::EdDSA,
      None,
      MethodScope::VerificationMethod,
    )
    .await?;

  write_to_file(&issuer_document, Some(path_did_file))?;
  println!("Web DID Document: {:#}", issuer_document);

  // Create a new client to interact with the IOTA ledger.
  let iota_client: Client = Client::builder()
  .with_primary_node(API_ENDPOINT, None)?
  .finish()
  .await?;

  // Create an identity for the holder, in this case also the subject.
  let mut secret_manager_alice: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password(Password::from("secure_password_2".to_owned()))
      .build(random_stronghold_path())?,
  );
  let storage_alice: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let (_, alice_document, fragment_alice): (Address, IotaDocument, String) =
    create_did(&iota_client, &mut secret_manager_alice, &storage_alice).await?;


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
    .create_credential_jwt(
      &credential,
      &storage_issuer,
      &fragment_issuer,
      &JwsSignatureOptions::default(),
      None,
    )
    .await?;

  // Before sending this credential to the holder the issuer wants to validate that some properties
  // of the credential satisfy their expectations.

  // Validate the credential's signature using the issuer's DID Document, the credential's semantic structure,
  // that the issuance date is not in the future and that the expiration date is not in the past:
  JwtCredentialValidator::with_signature_verifier(EdDSAJwsVerifier::default())
    .validate::<_, Object>(
      &credential_jwt,
      &issuer_document,
      &JwtCredentialValidationOptions::default(),
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
  let presentation: Presentation<Jwt> =
    PresentationBuilder::new(alice_document.id().to_url().into(), Default::default())
      .credential(credential_jwt)
      .build()?;

  // Create a JWT verifiable presentation using the holder's verification method
  // and include the requested challenge and expiry timestamp.
  let presentation_jwt: Jwt = alice_document
    .create_presentation_jwt(
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
  // - JWT verification of the presentation (including checking the requested challenge to mitigate replay attacks)
  // - JWT verification of the credentials.
  // - The presentation holder must always be the subject, regardless of the presence of the nonTransferable property
  // - The issuance date must not be in the future.

  let presentation_verifier_options: JwsVerificationOptions =
    JwsVerificationOptions::default().nonce(challenge.to_owned());

  let mut resolver: Resolver<IotaDocument> = Resolver::new();
  resolver.attach_iota_handler(iota_client);
  

  // Resolve the holder's document.
  let holder_did: CoreDID = JwtPresentationValidatorUtils::extract_holder(&presentation_jwt)?;
  let holder: IotaDocument = resolver.resolve(&holder_did).await?;

  // Validate presentation. Note that this doesn't validate the included credentials.
  let presentation_validation_options =
    JwtPresentationValidationOptions::default().presentation_verifier_options(presentation_verifier_options);
  let presentation: DecodedJwtPresentation<Jwt> = JwtPresentationValidator::with_signature_verifier(
    EdDSAJwsVerifier::default(),
  )
  .validate(&presentation_jwt, &holder, &presentation_validation_options)?;

  // Concurrently resolve the issuers' documents.
  let jwt_credentials: &Vec<Jwt> = &presentation.presentation.verifiable_credential;

  let mut resolver_web: Resolver<CoreDocument> = Resolver::new();
  let _ = resolver_web.attach_web_handler(client)?;

  let issuers: Vec<CoreDID> = jwt_credentials
    .iter()
    .map(JwtCredentialValidatorUtils::extract_issuer_from_jwt)
    .collect::<Result<Vec<CoreDID>, _>>()?;
  let issuers_documents: HashMap<CoreDID, CoreDocument> = resolver_web.resolve_multiple(&issuers).await?;

  // Validate the credentials in the presentation.
  let credential_validator: JwtCredentialValidator<EdDSAJwsVerifier> =
    JwtCredentialValidator::with_signature_verifier(EdDSAJwsVerifier::default());
  let validation_options: JwtCredentialValidationOptions = JwtCredentialValidationOptions::default()
    .subject_holder_relationship(holder_did.to_url().into(), SubjectHolderRelationship::AlwaysSubject);

  for (index, jwt_vc) in jwt_credentials.iter().enumerate() {
    // SAFETY: Indexing should be fine since we extracted the DID from each credential and resolved it.
    let issuer_document: &CoreDocument = &issuers_documents[&issuers[index]];

    let _decoded_credential: DecodedJwtCredential<Object> = credential_validator
      .validate::<_, Object>(jwt_vc, issuer_document, &validation_options, FailFast::FirstError)
      .unwrap();
  }

  // Since no errors were thrown by `verify_presentation` we know that the validation was successful.
  println!("VP successfully validated: {:#?}", presentation.presentation);


  Ok(())
}