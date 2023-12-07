// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example shows how to create a selective disclosure verifiable credential and validate it
//! using the standard [Selective Disclosure for JWTs (SD-JWT)](https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-06.html).
//!
//! cargo run --release --example 7_sd_jwt

use examples::create_did;
use examples::random_stronghold_path;
use examples::MemStorage;
use examples::API_ENDPOINT;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::core::json;
use identity_iota::core::FromJson;
use identity_iota::core::Object;
use identity_iota::core::ToJson;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::CredentialBuilder;
use identity_iota::credential::FailFast;
use identity_iota::credential::Jws;
use identity_iota::credential::JwtCredentialValidationOptions;
use identity_iota::credential::SdJwtValidator;
use identity_iota::credential::Subject;
use identity_iota::did::DID;
use identity_iota::document::verifiable::JwsVerificationOptions;
use identity_iota::iota::IotaDocument;
use identity_iota::storage::JwkDocumentExt;
use identity_iota::storage::JwkMemStore;
use identity_iota::storage::JwsSignatureOptions;
use identity_iota::storage::KeyIdMemstore;
use iota_sdk::client::secret::stronghold::StrongholdSecretManager;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::client::Client;
use iota_sdk::client::Password;
use iota_sdk::types::block::address::Address;
use sd_jwt::KeyBindingJwtClaims;
use sd_jwt::SdJwt;
use sd_jwt::SdObjectDecoder;
use sd_jwt::SdObjectEncoder;
use sd_jwt::Sha256Hasher;
use serde_json::to_string_pretty;

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

  // ===========================================================================
  // Step 2: Issuer creates and signs a selectively disclosable JWT verifiable credential.
  // ===========================================================================

  // Create a credential subject indicating the degree earned by Alice.
  let subject: Subject = Subject::from_json_value(json!({
    "id": alice_document.id().as_str(),
    "name": "Alice",
    "address": {
      "locality": "Maxstadt",
      "postal_code": "12344",
      "country": "DE",
      "street_address": "Weidenstraße 22"
    }
  }))?;

  // Build credential using subject above and issuer.
  let credential: Credential = CredentialBuilder::default()
    .id(Url::parse("https://example.edu/credentials/3732")?)
    .issuer(Url::parse(issuer_document.id().as_str())?)
    .type_("AddressCredential")
    .subject(subject)
    .build()?;

  // In Order to create an selective disclosure JWT, the plain text JWT
  // claims set must be created first.
  let payload = credential.serialize_jwt(None)?;
  println!("Claims set in plain text: {}", to_string_pretty(&payload)?);

  // Using the crate `sd-jwt` properties of the claims can be made selectively disclosable.
  // The default sha-256 hasher will be used to create the digests.
  // Read more in https://github.com/iotaledger/sd-jwt .
  let mut encoder = SdObjectEncoder::new(&payload)?;
  // Make "locality", "postal_code" and "street_address" selectively disclosable while keeping
  // other properties in plain text.
  let mut disclosures = vec![];
  disclosures.push(encoder.conceal(&["vc", "credentialSubject", "locality"], None)?);
  disclosures.push(encoder.conceal(&["vc", "credentialSubject", "postal_code"], None)?);
  disclosures.push(encoder.conceal(&["vc", "credentialSubject", "street_address"], None)?);
  encoder.add_sd_alg_property();
  let encoded_payload = encoder.try_to_string()?;

  println!(
    "Claims set with disclosure digests: {}",
    to_string_pretty(&encoded_payload)?
  );

  // Create the signed JWT.
  let jwt: Jws = issuer_document
    .create_jws(
      &issuer_storage,
      &fragment,
      encoded_payload.as_bytes(),
      &JwsSignatureOptions::default(),
    )
    .await?;

  // ===========================================================================
  // Step 3: Issuer sends the JWT and the disclosures to the holder.
  // ===========================================================================

  // One way to send the JWT and the disclosures, is by creating an SD-JWT with all the
  // disclosures.
  let disclosures: Vec<String> = disclosures
    .into_iter()
    .map(|disclosure| disclosure.to_string())
    .collect();
  let sd_jwt_str = SdJwt::new(jwt.into(), disclosures, None).presentation();

  // ===========================================================================
  // Step 4: Verifier sends the holder a challenge and requests a signed Verifiable Presentation.
  // ===========================================================================

  // A unique random challenge generated by the requester per presentation can mitigate replay attacks.
  let nonce: &str = "475a7984-1bb5-4c4c-a56f-822bccd46440";

  // ===========================================================================
  // Step 5: Holder creates an SD-JWT to be presented to a verifier.
  // ===========================================================================

  let sd_jwt = SdJwt::parse(&sd_jwt_str)?;

  // The holder only wants to present "locality" and "postal_code" but not "street_address".
  let disclosures = vec![
    sd_jwt.disclosures.get(0).unwrap().clone(),
    sd_jwt.disclosures.get(1).unwrap().clone(),
  ];

  // Optionally, the holder can add a Key Binding JWT. This is dependent on the verifier's policy.
  // Creating the Key Binding JWT is done by creating the claims set and setting the header `typ` value
  // with the help of `KeyBindingJwtClaims`.
  let binding_claims = KeyBindingJwtClaims::new(
    &Sha256Hasher::new(),
    sd_jwt.jwt.as_str().to_owned(),
    disclosures.clone(),
    nonce.to_string(),
    "did:example:verifier".to_string(),
    None,
  )
  .to_json()?;
  let kb_claims_bytes = binding_claims.as_bytes();
  let options = JwsSignatureOptions::new().typ(KeyBindingJwtClaims::KB_JWT_HEADER_TYP);
  let kb_jwt: Jws = alice_document
    .create_jws(&alice_storage, &alice_fragment, kb_claims_bytes, &options)
    .await?;

  let sd_jwt_obj = SdJwt::new(
    sd_jwt.jwt.as_str().to_owned(),
    disclosures,
    Some(kb_jwt.as_str().to_owned()),
  );

  // ===========================================================================
  // Step 6: Holder presents the SD-JWT to the verifier.
  // ===========================================================================

  let sd_jwt_presentation: String = sd_jwt_obj.presentation();

  // ===========================================================================
  // Step 7: Verifier receives the SD-JWT and verifies it.
  // ===========================================================================

  // The verifier wants the following requirements to be satisfied:
  // - JWT verification of the SD-JWT (including checking the requested challenge to mitigate replay attacks)
  // - JWT verification of the credential.
  // - The presentation holder must always be the subject.
  // - The issuance date must not be in the future.
  //

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
