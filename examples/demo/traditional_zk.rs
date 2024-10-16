use std::{collections::HashMap, fs::File, path::Path};
use env_logger::Env;
use examples::{create_did, random_stronghold_path, MemStorage, API_ENDPOINT};
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::{core::{Duration, FromJson, Object, Timestamp, Url}, credential::{Credential, CredentialBuilder, DecodedJwtCredential, DecodedJwtPresentation, FailFast, Jpt, JptCredentialValidationOptions, JptCredentialValidator, JptPresentationValidationOptions, JptPresentationValidator, JptPresentationValidatorUtils, JwpCredentialOptions, JwpPresentationOptions, Jwt, JwtCredentialValidationOptions, JwtCredentialValidator, JwtCredentialValidatorUtils, JwtPresentationOptions, JwtPresentationValidationOptions, JwtPresentationValidator, JwtPresentationValidatorUtils, Presentation, PresentationBuilder, SelectiveDisclosurePresentation, Subject, SubjectHolderRelationship}, did::{CoreDID, DIDJwk, DID}, document::{verifiable::JwsVerificationOptions, CoreDocument}, iota::{IotaDocument, NetworkName}, resolver::Resolver, storage::{DidJwkDocumentExt, JwkGenOutput, JwkMemStore, JwkStorage, JwpDocumentExt, JwsSignatureOptions, KeyIdMemstore, KeyIdStorage, KeyStorageResult, MethodDigest}, verification::{jws::JwsAlgorithm, jwu::encode_b64_json, MethodScope}};
use identity_iota::storage::JwkDocumentExt;
use iota_sdk::{client::{secret::{stronghold::StrongholdSecretManager, SecretManager}, Client, Password}, types::block::address::Address};
use jsonprooftoken::jpa::algs::ProofAlgorithm;
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
    //env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let did_url: &str = "https://localhost:4443/.well-known/did_zk.json";
    let path_did_file: &str = "C:/Projects/did-web-server/.well-known/did_zk.json";

    println!("{} {} {}", "[Issuer]".red(), ": Create DID (with did:web method) and publish the DID Document at", did_url);
  
    let client= ClientBuilder::new()
    .danger_accept_invalid_certs(true)
    .build()?;
  
    let mut issuer_document: CoreDocument = CoreDocument::new_from_url(did_url)?;

    let storage_issuer: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
    let fragment_issuer = issuer_document
      .generate_method_jwp(
        &storage_issuer,
        JwkMemStore::BLS12381G2_KEY_TYPE,
        ProofAlgorithm::BLS12381_SHA256,
        None,
        MethodScope::VerificationMethod,
      )
      .await?;
  
    write_to_file(&issuer_document, Some(path_did_file))?;

    let storage_alice: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());

    let (alice_document, fragment_alice) = CoreDocument::new_did_jwk_zk(
      &storage_alice, 
      JwkMemStore::BLS12381G2_KEY_TYPE, 
      ProofAlgorithm::BLS12381_SHA256
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

    println!("{} {} {}", "[Holder]".blue(), ": Inserted Credential subject information: ", serde_json::to_string_pretty(&subject)?);

    println!("{} {} {}", "[Holder]".blue(), " <-> [Issuer]".red(), ": Challenge-response protocol to authenticate Holder's DID");

    println!("{} {} ","[Issuer]".red(), ": Construct VC");
    
    let credential: Credential = CredentialBuilder::default()
      .id(Url::parse("https://example.edu/credentials/3732")?)
      .issuer(Url::parse(issuer_document.id().as_str())?)
      .type_("UniversityDegreeCredential")
      .subject(subject)
      .build()?;
  
    let credential_jpt: Jpt = issuer_document
      .create_credential_jpt(
        &credential,
        &storage_issuer,
        &fragment_issuer,
        &JwpCredentialOptions::default(),
        None,
      )
      .await?;


    println!("{} {} {} {}", "[Issuer]".red(), " -> [Holder]".blue(), ": Sending VC (as JPT):", credential_jpt.as_str());

    println!("{} {} {}", "[Holder]".blue(), ": Resolve Issuer's DID:", issuer_document.id().as_str());

    println!("{} {}", "[Holder]".blue(), ": Validate VC");

    let decoded_jpt = JptCredentialValidator::validate::<_, Object>(
        &credential_jpt,
        &issuer_document,
        &JptCredentialValidationOptions::default(),
        FailFast::FirstError,
      )
      .unwrap();
  
    println!("{} {}", "[Verifier]".green(),  "-> [Holder]: Send challenge");
  
    let challenge: &str = "475a7984-1bb5-4c4c-a56f-822bccd46440";
  
    let expires: Timestamp = Timestamp::now_utc().checked_add(Duration::minutes(10)).unwrap();

    println!("{}: Engages in the Selective Disclosure of credential's attributes", "[Holder]".blue());

    let method_id = decoded_jpt
    .decoded_jwp
    .get_issuer_protected_header()
    .kid()
    .unwrap();

    let mut selective_disclosure_presentation = SelectiveDisclosurePresentation::new(&decoded_jpt.decoded_jwp);
    selective_disclosure_presentation
        .conceal_in_subject("degree.name")
        .unwrap();
  
    println!("{} {}", "[Holder]".blue(), ": Compute the Signature Proof of Knowledge and construct the Presentation JPT");
    
    let presentation_jpt: Jpt = issuer_document
        .create_presentation_jpt(
        &mut selective_disclosure_presentation,
        method_id,
        &JwpPresentationOptions::default().nonce(challenge),
        )
        .await?;

    println!("{} {} {} {}", "[Holder]".blue(), " -> [Verifier]".green(),  ": Sending Presentation (as JPT):", presentation_jpt.as_str());
  
    println!("{}: Resolve Issuer's DID and verifies the Presentation JPT","[Verifier]".green());
    
    let mut resolver_web: Resolver<CoreDocument> = Resolver::new();
    let _ = resolver_web.attach_web_handler(client)?;
  
    let issuer: CoreDID = JptPresentationValidatorUtils::extract_issuer_from_presented_jpt(&presentation_jpt).unwrap();
    let issuer_document: CoreDocument = resolver_web.resolve(&issuer).await?;
  
    let presentation_validation_options = JptPresentationValidationOptions::default().nonce(challenge);

    // Verifier validate the Presented Credential and retrieve the JwpPresented
    let decoded_presented_credential = JptPresentationValidator::validate::<_, Object>(
        &presentation_jpt,
        &issuer_document,
        &presentation_validation_options,
        FailFast::FirstError,
    )
    .unwrap();
  
    Ok(())
}
