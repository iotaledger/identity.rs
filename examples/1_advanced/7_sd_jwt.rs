// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example shows how to create a selective disclosure verifiable credential and validate it
//! using the standard [Selective Disclosure for JWTs (SD-JWT)](https://www.ietf.org/archive/id/draft-ietf-oauth-selective-disclosure-jwt-07.html).
//!
//! cargo run --release --example 7_sd_jwt

use examples::create_did;
use examples::pretty_print_json;
use examples::random_stronghold_path;
use examples::MemStorage;
use examples::API_ENDPOINT;
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::core::json;
use identity_iota::core::FromJson;
use identity_iota::core::Object;
use identity_iota::core::Timestamp;
use identity_iota::core::ToJson;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::CredentialBuilder;
use identity_iota::credential::FailFast;
use identity_iota::credential::Jws;
use identity_iota::credential::JwtCredentialValidationOptions;
use identity_iota::credential::KeyBindingJWTValidationOptions;
use identity_iota::credential::SdJwtCredentialValidator;
use identity_iota::credential::Subject;
use identity_iota::did::DID;
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
use sd_jwt_payload::KeyBindingJwtClaims;
use sd_jwt_payload::SdJwt;
use sd_jwt_payload::SdObjectDecoder;
use sd_jwt_payload::SdObjectEncoder;
use sd_jwt_payload::Sha256Hasher;

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

  // Create an address credential subject.
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
    .id(Url::parse("https://example.com/credentials/3732")?)
    .issuer(Url::parse(issuer_document.id().as_str())?)
    .type_("AddressCredential")
    .subject(subject)
    .build()?;

  // In Order to create an selective disclosure JWT, the plain text JWT
  // claims set must be created first.
  let payload = credential.serialize_jwt(None)?;
  pretty_print_json("Claims set in plain text", &payload);

  // Using the crate `sd-jwt` properties of the claims can be made selectively disclosable.
  // The default sha-256 hasher will be used to create the digests.
  // Read more in https://github.com/iotaledger/sd-jwt-payload .
  let mut encoder = SdObjectEncoder::new(&payload)?;
  // Make "locality", "postal_code" and "street_address" selectively disclosable while keeping
  // other properties in plain text.
  let disclosures = vec![
    encoder.conceal(&["vc", "credentialSubject", "address", "locality"], None)?,
    encoder.conceal(&["vc", "credentialSubject", "address", "postal_code"], None)?,
    encoder.conceal(&["vc", "credentialSubject", "address", "street_address"], None)?,
  ];
  encoder.add_sd_alg_property();
  let encoded_payload = encoder.try_to_string()?;

  pretty_print_json("Claims set with disclosure digests", &encoded_payload);

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

  const VERIFIER_DID: &str = "did:example:verifier";
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

  // Optionally, the holder can add a Key Binding JWT (KB-JWT). This is dependent on the verifier's policy.
  // Issuing the KB-JWT is done by creating the claims set and setting the header `typ` value
  // with the help of `KeyBindingJwtClaims`.
  let binding_claims = KeyBindingJwtClaims::new(
    &Sha256Hasher::new(),
    sd_jwt.jwt.as_str().to_string(),
    disclosures.clone(),
    nonce.to_string(),
    VERIFIER_DID.to_string(),
    Timestamp::now_utc().to_unix(),
  )
  .to_json()?;

  // Setting the `typ` in the header is required.
  let options = JwsSignatureOptions::new().typ(KeyBindingJwtClaims::KB_JWT_HEADER_TYP);

  // Create the KB-JWT.
  let kb_jwt: Jws = alice_document
    .create_jws(&alice_storage, &alice_fragment, binding_claims.as_bytes(), &options)
    .await?;

  // Create the final SD-JWT.
  let sd_jwt_obj = SdJwt::new(sd_jwt.jwt, disclosures, Some(kb_jwt.into()));

  // ===========================================================================
  // Step 6: Holder presents the SD-JWT to the verifier.
  // ===========================================================================

  let sd_jwt_presentation: String = sd_jwt_obj.presentation();

  // ===========================================================================
  // Step 7: Verifier receives the SD-JWT and verifies it.
  // ===========================================================================

  let sd_jwt = SdJwt::parse(&sd_jwt_presentation)?;

  // Verify the JWT.
  let decoder = SdObjectDecoder::new_with_sha256();
  let validator = SdJwtCredentialValidator::with_signature_verifier(EdDSAJwsVerifier::default(), decoder);
  let validation = validator
    .validate_credential::<_, Object>(
      &sd_jwt,
      &issuer_document,
      &JwtCredentialValidationOptions::default(),
      FailFast::FirstError,
    )
    .unwrap();

  println!("JWT successfully validated");
  pretty_print_json("Decoded Credential", &validation.credential.to_string());

  // Verify the Key Binding JWT.
  let options = KeyBindingJWTValidationOptions::new().nonce(nonce).aud(VERIFIER_DID);
  let _kb_validation = validator.validate_key_binding_jwt(&sd_jwt_obj, &alice_document, &options)?;

  println!("Key Binding JWT successfully validated");

  Ok(())
}
