// Copyright 2024 Fondazione Links
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashMap, fs::File, path::Path};
use examples::{MemStorage, DID_URL, PATH_DID_FILE};
use identity_iota::{core::{Duration, FromJson, Object, Timestamp, Url}, credential::{Credential, CredentialBuilder, DecodedJwtCredential, DecodedJwtPresentation, FailFast, Jwt, JwtCredentialValidationOptions, JwtCredentialValidator, JwtCredentialValidatorUtils, JwtPresentationOptions, JwtPresentationValidationOptions, JwtPresentationValidator, JwtPresentationValidatorUtils, Presentation, PresentationBuilder, Subject, SubjectHolderRelationship}, did::{CoreDID, DIDJwk, DID}, document::{verifiable::JwsVerificationOptions, CoreDocument}, resolver::Resolver, storage::{DidJwkDocumentExt, JwkMemStore, JwsDocumentExtPQC, JwsSignatureOptions, KeyIdMemstore}, verification::{jws::JwsAlgorithm, MethodScope}};
use identity_pqc_verifier::PQCJwsVerifier;
use reqwest::ClientBuilder;
use serde_json::json;
use colored::Colorize;

pub fn write_to_file(doc: &CoreDocument, path: Option<&str>) -> anyhow::Result<()> {
    let path = Path::new(path.unwrap_or_else(|| "did.json"));
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, doc)?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let binding = DID_URL.to_owned() + "did_pqc.json";
  let did_url: &str = binding.as_str();
  let binding = PATH_DID_FILE.to_owned() + "did_pqc.json";
  let path_did_file: &str = binding.as_str();
  
  println!("{} {} {}", "[Issuer]".red(), ": Create DID (with did:web method) and publish the DID Document at", did_url);

  let client= ClientBuilder::new()
  .danger_accept_invalid_certs(true)
  .build()?;

  let mut issuer_document: CoreDocument = CoreDocument::new_from_url(did_url)?;

  let storage_issuer: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let fragment_issuer = issuer_document
  .generate_method_pqc(
    &storage_issuer,
    JwkMemStore::ML_DSA_KEY_TYPE,
    JwsAlgorithm::ML_DSA_44,
    None,
    MethodScope::VerificationMethod,
  ).await?;

  write_to_file(&issuer_document, Some(path_did_file))?;

  let storage_alice: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());

  let (alice_document, fragment_alice) = CoreDocument::new_did_jwk_pqc(
    &storage_alice, 
    JwkMemStore::ML_DSA_KEY_TYPE, 
    JwsAlgorithm::ML_DSA_44
  ).await?;

  println!("{} {} {}", "[Holder]".blue(), ": Create DID Jwk:", alice_document.id().as_str());

  let subject: Subject = Subject::from_json_value(json!({
    "id": alice_document.id().as_str(),
    "name": "Alice",
    "degree": {
      "type": "BachelorDegree",
      "name": "Bachelor of Science and Arts",
    },
    "GPA": "4.0",
  }))?;

  println!("{} {} {} {}", "[Holder]".blue(), "->", "[Issuer]".red(), ": Request Verifiable Credential (VC)");

  println!("{} {} {}", "[Holder]".blue(), ": Credential information: ", serde_json::to_string_pretty(&subject)?);

  println!("{} {} {} {}", "[Holder]".blue(), "<->", "[Issuer]".red(), ": Challenge-response protocol to authenticate Holder's DID");
  
  let credential: Credential = CredentialBuilder::default()
  .id(Url::parse("https://example.edu/credentials/3732")?)
  .issuer(Url::parse(issuer_document.id().as_str())?)
  .type_("UniversityDegreeCredential")
  .subject(subject)
  .build()?;

  let credential_jwt: Jwt = issuer_document.create_credential_jwt_pqc(
    &credential,
    &storage_issuer,
    &fragment_issuer,
    &JwsSignatureOptions::default(),
    None,
  ).await?;

  println!("{} {} {}","[Issuer]".red(), ": Generate VC (JWT encoded): ", credential_jwt.as_str());

  println!("{} {} {} {}", "[Issuer]".red(), "->", "[Holder]".blue(), ": Sending VC");

  println!("{} {} {}", "[Holder]".blue(), ": Resolve Issuer's DID:", issuer_document.id().as_str());

  println!("{} {} {issuer_document:#}", "[Holder]".blue(), ": Issuer's DID Document:");

  println!("{} {}", "[Holder]".blue(), ": Verify VC");

  JwtCredentialValidator::with_signature_verifier(PQCJwsVerifier::default())
  .validate::<_, Object>(
    &credential_jwt,
    &issuer_document,
    &JwtCredentialValidationOptions::default(),
    FailFast::FirstError,
  ).unwrap();

  println!("{} {}", "[Holder]".blue(), ": Successfull verification");

  println!("{} {} {} {}", "[Holder]".blue(), "->", "[Verifier]".green(), ": Request access");

  let challenge: &str = "475a7984-1bb5-4c4c-a56f-822bccd46440";

  println!("{} {} {} {} {}", "[Verifier]".green(),  "->",  "[Holder]".blue(), ": Send challenge: ", challenge);

  let expires: Timestamp = Timestamp::now_utc().checked_add(Duration::minutes(10)).unwrap();
  
  let presentation: Presentation<Jwt> =PresentationBuilder::new(
    alice_document.id().to_url().into(),
    Default::default()
  ).credential(credential_jwt).build()?;

  let presentation_jwt: Jwt = alice_document.create_presentation_jwt_pqc(
    &presentation,
    &storage_alice,
    &fragment_alice,
    &JwsSignatureOptions::default().nonce(challenge.to_owned()),
    &JwtPresentationOptions::default().expiration_date(expires),
  ).await?;

  println!("{} {} {}", "[Holder]".blue(), ": Generate Verifiable Presentation (VP) (JWT encoded) :", presentation_jwt.as_str());
  
  println!("{} {} {} {}", "[Holder]".blue(), "->", "[Verifier]".green(),  ": Sending VP");

  println!("{}: Resolve Issuer's DID and Holder's DID to verify the VP", "[Verifier]".green());

  let mut resolver: Resolver<CoreDocument> = Resolver::new();
  resolver.attach_did_jwk_handler();
  
  let holder_did: DIDJwk = JwtPresentationValidatorUtils::extract_holder::<DIDJwk>(&presentation_jwt)?;
  let holder: CoreDocument = resolver.resolve(&holder_did).await?;

  let presentation_verifier_options: JwsVerificationOptions =
  JwsVerificationOptions::default().nonce(challenge.to_owned());

  let presentation_validation_options =
    JwtPresentationValidationOptions::default().presentation_verifier_options(presentation_verifier_options);
  let presentation: DecodedJwtPresentation<Jwt> = JwtPresentationValidator::with_signature_verifier(
    PQCJwsVerifier::default(),
  )
  .validate(&presentation_jwt, &holder, &presentation_validation_options)?;

  let jwt_credentials: &Vec<Jwt> = &presentation.presentation.verifiable_credential;

  let mut resolver_web: Resolver<CoreDocument> = Resolver::new();
  let _ = resolver_web.attach_web_handler(client)?;

  let issuers: Vec<CoreDID> = jwt_credentials
    .iter()
    .map(JwtCredentialValidatorUtils::extract_issuer_from_jwt)
    .collect::<Result<Vec<CoreDID>, _>>()?;
  let issuers_documents: HashMap<CoreDID, CoreDocument> = resolver_web.resolve_multiple(&issuers).await?;

  let credential_validator: JwtCredentialValidator<PQCJwsVerifier> =
    JwtCredentialValidator::with_signature_verifier(PQCJwsVerifier::default());
  let validation_options: JwtCredentialValidationOptions = JwtCredentialValidationOptions::default()
    .subject_holder_relationship(holder_did.to_url().into(), SubjectHolderRelationship::AlwaysSubject);

  for (index, jwt_vc) in jwt_credentials.iter().enumerate() {
    let issuer_document: &CoreDocument = &issuers_documents[&issuers[index]];

    let _decoded_credential: DecodedJwtCredential<Object> = credential_validator
      .validate::<_, Object>(jwt_vc, issuer_document, &validation_options, FailFast::FirstError)
      .unwrap();
  }

  println!("{}: VP successfully verified, access granted", "[Verifier]".green());

  Ok(())
}
