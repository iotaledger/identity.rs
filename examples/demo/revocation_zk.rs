// Copyright 2024 Fondazione Links
// SPDX-License-Identifier: Apache-2.0

use std::{fs::File, path::Path};
use examples::{MemStorage, DID_URL, PATH_DID_FILE};
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::{core::{Duration, FromJson, Object, Timestamp, Url}, credential::{Credential, CredentialBuilder, DecodedJwtPresentation, FailFast, Jpt, JptCredentialValidationOptions, JptCredentialValidator, JptCredentialValidatorUtils, JptPresentationValidationOptions, JptPresentationValidator, JptPresentationValidatorUtils, JwpCredentialOptions, JwpPresentationOptions, Jwt, JwtPresentationOptions, JwtPresentationValidationOptions, JwtPresentationValidator, JwtPresentationValidatorUtils, Presentation, PresentationBuilder, RevocationBitmap, RevocationDocumentExt, RevocationTimeframeStatus, SelectiveDisclosurePresentation, Status, StatusCheck, Subject, SubjectHolderRelationship}, did::{CoreDID, DIDUrl, DID}, document::{verifiable::JwsVerificationOptions, CoreDocument, Service}, resolver::Resolver, storage::{DidJwkDocumentExt, JwkDocumentExt, JwkMemStore, JwpDocumentExt, JwsSignatureOptions, KeyIdMemstore, TimeframeRevocationExtension}, verification::{jws::JwsAlgorithm, MethodScope}};
use jsonprooftoken::jpa::algs::ProofAlgorithm;
use reqwest::ClientBuilder;
use serde_json::json;
use colored::Colorize;
use std::time::Duration as SleepDuration;
use std::thread;

pub fn write_to_file(doc: &CoreDocument, path: Option<&str>) -> anyhow::Result<()> {
    let path = Path::new(path.unwrap_or_else(|| "did.json"));
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, doc)?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let binding = DID_URL.to_owned() + "did_zk.json";
  let did_url: &str = binding.as_str();
  let binding = PATH_DID_FILE.to_owned() + "did_zk.json";
  let path_did_file: &str = binding.as_str();

  println!("{} {} {}", "[Issuer]".red(), ": Create DID with the Revocationbitmap (with did:web method) and publish the DID Document at", did_url);

  let client= ClientBuilder::new()
  .danger_accept_invalid_certs(true)
  .build()?;

  let mut issuer_document: CoreDocument = CoreDocument::new_from_url(did_url)?;

  let storage_issuer: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let fragment_issuer = issuer_document.generate_method_jwp(
    &storage_issuer,
    JwkMemStore::BLS12381G2_KEY_TYPE,
    ProofAlgorithm::BLS12381_SHA256,
    None,
    MethodScope::VerificationMethod,
  ).await?;

  let revocation_bitmap_issuer: RevocationBitmap = RevocationBitmap::new();

  let service_id: DIDUrl = issuer_document.id().to_url().join("#my-revocation-service")?;
  let service: Service = revocation_bitmap_issuer.to_service(service_id)?;

  issuer_document.insert_service(service).unwrap();

  write_to_file(&issuer_document, Some(path_did_file))?;

  let storage_alice: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());

  let (mut alice_document, fragment_alice) = CoreDocument::new_did_jwk(
    &storage_alice, 
    JwkMemStore::ED25519_KEY_TYPE, 
    JwsAlgorithm::EdDSA
  ).await?;

  let revocation_bitmap_holder: RevocationBitmap = RevocationBitmap::new();

  let service_id: DIDUrl = alice_document.id().to_url().join("#my-revocation-service")?;
  let service: Service = revocation_bitmap_holder.to_service(service_id)?;

  alice_document.insert_service(service).unwrap();

  println!("{} {} {}", "[Holder]".blue(), ": Create DID Jwk with the Revocationbitmap:", alice_document.id().as_str());

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

  println!("{} {} ","[Issuer]".red(), ": Create a new timeframe of 30 seconds");

  let duration = Duration::seconds(30);
  
  let service_url = issuer_document.id().to_url().join("#my-revocation-service")?;
  let credential_index: u32 = 5;

  let start_validity_timeframe = Timestamp::now_utc();
  let status: Status = RevocationTimeframeStatus::new(
    Some(start_validity_timeframe),
    duration,
    service_url.into(),
    credential_index,
  )?
  .into();
  
  println!("{} {} ","[Issuer]".red(), ": Generate VC with timeframe");
  
  let credential: Credential = CredentialBuilder::default()
    .id(Url::parse("https://example.edu/credentials/3732")?)
    .issuer(Url::parse(issuer_document.id().as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .status(status)
    .build()?;

  let credential_jpt: Jpt = issuer_document.create_credential_jpt(
    &credential,
    &storage_issuer,
    &fragment_issuer,
    &JwpCredentialOptions::default(),
    None,
  ).await?;

  println!("{} {} {} {}", "[Issuer]".red(), " -> [Holder]".blue(), ": Sending VC (as JPT):", credential_jpt.as_str());

  println!("{} {} {}", "[Holder]".blue(), ": Resolve Issuer's DID:", issuer_document.id().as_str());

  println!("{} {} {issuer_document:#}", "[Holder]".blue(), ": Issuer's DID Document:");

  println!("{} {}", "[Holder]".blue(), ": Validate VC");

  let decoded_jpt = JptCredentialValidator::validate::<_, Object>(
    &credential_jpt,
    &issuer_document,
    &JptCredentialValidationOptions::default(),
    FailFast::FirstError,
  ).unwrap();

  println!("{} {}", "[Holder]".blue(), ": Validate Timeframe and revocation");

  let _revocation_result = JptCredentialValidatorUtils::check_timeframes_and_revocation_with_validity_timeframe_2024(
    &decoded_jpt.credential,
    &issuer_document,
    None,
    StatusCheck::Strict,
  ).unwrap();

  println!("{} {}", "[Holder]".blue(), ": Successfull verification");

  println!("{} {} {} {}", "[Holder]".blue(), "->", "[Verifier]".green(), ": Request access with Selective Disclosure of VC attributes");

  let challenge: &str = "475a7984-1bb5-4c4c-a56f-822bccd46440";

  println!("{} {} {} {} {}", "[Verifier]".green(),  "->",  "[Holder]".blue(), ": Send challenge:", challenge);

  println!("{} : Resolve Issuer's Public Key to compute the Signature Proof of Knowledge", "[Holder]".blue());

  let method_id = decoded_jpt
  .decoded_jwp
  .get_issuer_protected_header()
  .kid()
  .unwrap();

  println!("{} : Engages in the Selective Disclosure of credential's attributes GPA and name", "[Holder]".blue());

  let mut selective_disclosure_presentation = SelectiveDisclosurePresentation::new(&decoded_jpt.decoded_jwp);
  selective_disclosure_presentation
  .conceal_in_subject("GPA")
  .unwrap();

  selective_disclosure_presentation.conceal_in_subject("name").unwrap();

  println!("{} {}", "[Holder]".blue(), ": Compute the Signature Proof of Knowledge and generate the Presentation/zk_proof (JPT encoded)");
  
  let presentation_jpt: Jpt = issuer_document
  .create_presentation_jpt(
    &mut selective_disclosure_presentation,
    method_id,
    &JwpPresentationOptions::default().nonce(challenge),
    )
  .await?;

  println!("{} {} {} {} {}", "[Holder]".blue(), "->",  "[Verifier]".green(),  ": Sending Presentation (as JPT):", presentation_jpt.as_str());

  println!("{} : Resolve Issuer's DID and verifies the Presentation/zk_proof (JPT encoded)","[Verifier]".green());
  
  let mut resolver_web: Resolver<CoreDocument> = Resolver::new();
  let _ = resolver_web.attach_web_handler(client)?;

  let issuer: CoreDID = JptPresentationValidatorUtils::extract_issuer_from_presented_jpt(&presentation_jpt).unwrap();
  let resolved_issuer_document: CoreDocument = resolver_web.resolve(&issuer).await?;

  let presentation_validation_options = JptPresentationValidationOptions::default().nonce(challenge);

  let decoded_presented_credential = JptPresentationValidator::validate::<_, Object>(
    &presentation_jpt,
    &resolved_issuer_document,
    &presentation_validation_options,
    FailFast::FirstError,
  ).unwrap();

  println!("{} : Verify the validity of timeframe","[Verifier]".green());

  let _timeframe_result = JptPresentationValidatorUtils::check_timeframes_with_validity_timeframe_2024(
    &decoded_presented_credential.credential,
    None,
    StatusCheck::Strict,
  ).unwrap();

  println!("{}: JPT successfully verified, access granted", "[Verifier]".green());

  println!("");

  println!("Waiting for the next validityTimeframe");

  println!("");

  thread::sleep(SleepDuration::from_secs(31));

  println!("{} {} {} {}", "[Holder]".blue(), "->", "[Verifier]".green(), ": Request access after Timeframe expiration");

  let timeframe_result = JptPresentationValidatorUtils::check_timeframes_with_validity_timeframe_2024(
    &decoded_presented_credential.credential,
    None,
    StatusCheck::Strict,
  );

  println!("{} {} {}", "[Verifier]".green(), ": Verify the validity of timeframe :", timeframe_result.unwrap_err());

  println!("{} {} {} {}", "[Holder]".blue(), "->", "[Issuer]".red(), ": Request update Timeframe");

  println!("{} {} {} {}", "[Issuer]".red(), "->", "[Holder]".blue(), ": Sending a challenge");

  let expires: Timestamp = Timestamp::now_utc().checked_add(Duration::minutes(10)).unwrap();

  let presentation: Presentation<Jpt> =
  PresentationBuilder::new(alice_document.id().to_url().into(), Default::default())
    .credential(credential_jpt)
    .build()?;

  let presentation_jwt: Jwt = alice_document
  .create_presentation_jwt(
    &presentation,
    &storage_alice,
    &fragment_alice,
    &JwsSignatureOptions::default().nonce(challenge.to_owned()),
    &JwtPresentationOptions::default().expiration_date(expires),
  ).await?;

  println!("{} {} ","[Issuer]".red(), ": Generate a Verifiable Presentation (VP) from the expired VC including the challenge and a new expiry timestamp");

  println!("{} {} {} {} {}", "[Holder]".blue(), "->", "[Issuer]".red(), ": Sending VP (as JWT):", presentation_jwt.as_str());

  println!("{} {} ","[Issuer]".red(), ": Resolve Issuer's DID and verify the VP");

  let presentation_verifier_options: JwsVerificationOptions =
  JwsVerificationOptions::default().nonce(challenge.to_owned());

  let mut resolver_jwk: Resolver<CoreDocument> = Resolver::new();
  let _ = resolver_jwk.attach_did_jwk_handler();

  let holder_did: CoreDID = JwtPresentationValidatorUtils::extract_holder(&presentation_jwt)?;
  let holder:CoreDocument  = resolver_jwk.resolve(&holder_did).await?;

  let presentation_validation_options =
  JwtPresentationValidationOptions::default().presentation_verifier_options(presentation_verifier_options);
  let presentation: DecodedJwtPresentation<Jpt> = JwtPresentationValidator::with_signature_verifier(
    EdDSAJwsVerifier::default(),
  ).validate(&presentation_jwt, &holder, &presentation_validation_options)?;

  println!("{} {} ","[Issuer]".red(), ": Verify the ZK VC inside the VP");

  let validation_options: JptCredentialValidationOptions = JptCredentialValidationOptions::default()
  .subject_holder_relationship(holder_did.to_url().into(), SubjectHolderRelationship::AlwaysSubject);

  let jpt_credentials: &Vec<Jpt> = &presentation.presentation.verifiable_credential;

  let jpt_vc = jpt_credentials.first().unwrap();

  let mut verified_credential_result =
  JptCredentialValidator::validate::<_, Object>(jpt_vc, &issuer_document, &validation_options, FailFast::FirstError)
    .unwrap();

  let _revocation_result = JptCredentialValidatorUtils::check_revocation_with_validity_timeframe_2024(
    &verified_credential_result.credential,
    &issuer_document,
    StatusCheck::Strict,
  ).unwrap();

  println!("{} {} ","[Issuer]".red(), ": VP successfully validated");

  println!("{} {} ","[Issuer]".red(), ": Update credential with new Timeframe");

  let new_credential_jpt = issuer_document
  .update(
    &storage_issuer,
    &fragment_issuer,
    None,
    duration,
    &mut verified_credential_result.decoded_jwp,
  ).await?;

  println!("{} {} {} {} {}", "[Issuer]".red(), "->", "[Holder]".blue(), ": Sending updated credential (as JPT)", new_credential_jpt.as_str());

  println!("{} {}", "[Holder]".blue(), ": Validate updated VC");

  let decoded_jpt = JptCredentialValidator::validate::<_, Object>(
    &new_credential_jpt,
    &issuer_document,
    &JptCredentialValidationOptions::default(),
    FailFast::FirstError,
  ).unwrap();

  let _ = JptCredentialValidatorUtils::check_timeframes_with_validity_timeframe_2024(
    &decoded_jpt.credential,
    None,
    StatusCheck::Strict,
  ).unwrap();

  println!("{} {}", "[Holder]".blue(), ": Successfull verification");

  println!("{} {} {} {}", "[Holder]".blue(), "->", "[Verifier]".green(), ": Request access");

  let challenge: &str = "7788554-2598-ff55-ef52-822888d464dd";

  println!("{} {} {} {} {}", "[Verifier]".green(),  "->",  "[Holder]".blue(), ": Send challenge:", challenge);

  println!("{} {}", "[Holder]".blue(), ": Compute the Signature Proof of Knowledge and generate the Presentation/zk_proof (JPT encoded) from the updated VC");

  let mut selective_disclosure_presentation = SelectiveDisclosurePresentation::new(&decoded_jpt.decoded_jwp);
  selective_disclosure_presentation
  .conceal_in_subject("GPA")
  .unwrap();

  selective_disclosure_presentation.conceal_in_subject("name").unwrap();

  let method_id = decoded_jpt
  .decoded_jwp
  .get_issuer_protected_header()
  .kid()
  .unwrap();

  let updated_presentation_jpt: Jpt = issuer_document
  .create_presentation_jpt(
    &mut selective_disclosure_presentation,
    method_id,
    &JwpPresentationOptions::default().nonce(challenge),
  ).await?;

  println!("{} {} {} {} {}", "[Holder]".blue(), "->",  "[Verifier]".green(),  ": Sending Presentation (as JPT):", updated_presentation_jpt.as_str());

  println!("{} : Resolve Issuer's DID and verifies the Presentation/zk_proof (JPT encoded)","[Verifier]".green());
  
  let presentation_validation_options = JptPresentationValidationOptions::default().nonce(challenge);

  let decoded_presented_credential = JptPresentationValidator::validate::<_, Object>(
    &updated_presentation_jpt,
    &resolved_issuer_document,
    &presentation_validation_options,
    FailFast::FirstError,
  ).unwrap();

  println!("{} : Verify the validity of timeframe","[Verifier]".green());

  let _timeframe_result = JptPresentationValidatorUtils::check_timeframes_with_validity_timeframe_2024(
    &decoded_presented_credential.credential,
    None,
    StatusCheck::Strict,
  ).unwrap();

  println!("{} : JPT successfully verified, access granted", "[Verifier]".green());

  println!("{} {} ","[Issuer]".red(), ": Decide to revoke the Holder's Credential");

  println!("{} {} {}", "[Issuer]".red(), ": Update the Bitmap and publish the DID Document (with did:web method) at", did_url);
  
  issuer_document.revoke_credentials("my-revocation-service", &[credential_index])?;

  write_to_file(&issuer_document, Some(path_did_file))?;

  println!("{} {} {} {}", "[Holder]".blue(), "->", "[Verifier]".green(), ": Request access (after Issuer revoke)");

  println!("{} {} {} {}", "[Verifier]".green(),  "->",  "[Holder]".blue(), ": Resolve Issuer's DID and verifies the Presentation");

  let client= ClientBuilder::new()
  .danger_accept_invalid_certs(true)
  .build()?;

  let mut resolver_web: Resolver<CoreDocument> = Resolver::new();
  let _ = resolver_web.attach_web_handler(client)?; 

  let issuer: CoreDID = JptPresentationValidatorUtils::extract_issuer_from_presented_jpt(&updated_presentation_jpt).unwrap();
  let resolved_issuer_document: CoreDocument = resolver_web.resolve(&issuer).await?;

  let decoded_jpt = JptCredentialValidator::validate::<_, Object>(
    &new_credential_jpt,
    &resolved_issuer_document,
    &JptCredentialValidationOptions::default(),
    FailFast::FirstError,
  ).unwrap();

  let revocation_result = JptCredentialValidatorUtils::check_revocation_with_validity_timeframe_2024(
    &decoded_jpt.credential,
    &resolved_issuer_document,
    StatusCheck::Strict,
  );

  println!("{} {} {}", "[Verifier]".green(), " : Error ", revocation_result.unwrap_err());

  Ok(())
}
