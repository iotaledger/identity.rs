// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example shows how to create an SD-JWT Verifiable Credential and validate it.
//!
//! cargo run --release --example 7_sd_jwt

use examples::create_did;
use examples::MemStorage;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::core::Object;

use identity_iota::core::Timestamp;
use identity_iota::core::ToJson;
use identity_iota::credential::Jws;
use identity_iota::credential::JwtCredentialValidationOptions;
use identity_iota::credential::SdJwtValidator;
use identity_iota::document::verifiable::JwsVerificationOptions;
use identity_iota::storage::JwkDocumentExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::JwsSignatureOptions;
use identity_iota::storage::KeyIdMemstore;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::client::Password;
use iota_sdk::types::block::address::Address;

use examples::random_stronghold_path;
use examples::API_ENDPOINT;
use identity_iota::core::json;
use identity_iota::core::FromJson;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::CredentialBuilder;
use identity_iota::credential::FailFast;
use identity_iota::credential::Subject;
use identity_iota::did::DID;
use identity_iota::iota::IotaDocument;
use sd_jwt::KeyBindingJwtClaims;
use sd_jwt::SdJwt;
use sd_jwt::SdObjectDecoder;
use sd_jwt::SdObjectEncoder;
use sd_jwt::Sha256Hasher;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder()
    .with_primary_node(API_ENDPOINT, None)?
    .finish()
    .await?;

  // Create an identity for the issuer with one verification method `key-1`.
  let mut secret_manager_issuer: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password(Password::from("secure_password_1".to_owned()))
      .build(random_stronghold_path())?,
  );
  let issuer_storage: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let (_, issuer_document, fragment): (Address, IotaDocument, String) =
    create_did(&client, &mut secret_manager_issuer, &issuer_storage).await?;

  // Create an identity for the holder, in this case also the subject.
  let mut secret_manager_alice: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password(Password::from("secure_password_2".to_owned()))
      .build(random_stronghold_path())?,
  );
  let alice_storage: MemStorage = MemStorage::new(JwkMemStore::new(), KeyIdMemstore::new());
  let (_, alice_document, alice_fragment): (Address, IotaDocument, String) =
    create_did(&client, &mut secret_manager_alice, &alice_storage).await?;

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

  let payload = credential.serialize_jwt(None)?;
  let mut encoder = SdObjectEncoder::new(&payload)?;
  encoder.add_sd_alg_property();
  let disclosure = encoder.conceal(&["vc", "credentialSubject"], None)?;
  let encoded_payload = encoder.try_to_string()?;

  let jwt: Jws = issuer_document
    .create_jws(
      &issuer_storage,
      &fragment,
      encoded_payload.as_bytes(),
      &JwsSignatureOptions::default(),
    )
    .await?;

  let binding_claims = KeyBindingJwtClaims::new(
    &Sha256Hasher::new(),
    jwt.as_str().to_owned(),
    vec![disclosure.to_string()],
    "abc.efd".to_string(),
    "abc123".to_string(),
    None,
  )
  .to_json()?;
  let kb_claims_bytes = binding_claims.as_bytes();
  let options = JwsSignatureOptions::new().typ("kb-jwt");
  let jws = alice_document
    .create_jws(&alice_storage, &alice_fragment, kb_claims_bytes, &options)
    .await?;

  let sd_jwt_obj = SdJwt::new(
    jwt.as_str().to_owned(),
    vec![disclosure.to_string()],
    Some(jws.as_str().to_owned()),
  );

  let decoder = SdObjectDecoder::new();
  let validator = SdJwtValidator::new(EdDSAJwsVerifier::default(), decoder);
  let _validation = validator.validate_credential::<_, Object>(
    &sd_jwt_obj,
    &issuer_document,
    &JwtCredentialValidationOptions::default(),
    FailFast::FirstError,
  )?;
  println!("VC successfully validated");

  let _kb_validation = validator.validate_key_binding_jwt(
    &sd_jwt_obj,
    &alice_document,
    "abc123".to_string(),
    Some("abc.efd".to_string()),
    &JwsVerificationOptions::default(),
    None,
  )?;

  println!("Key Binding JWT successfully validated");

  Ok(())
}
