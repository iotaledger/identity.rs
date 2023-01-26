// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example shows how to create a Verifiable Credential and validate it.
//! In this example, alice takes the role of the subject, while we also have an issuer.
//! The issuer signs a UniversityDegreeCredential type verifiable credential with Alice's name and DID.
//! This Verifiable Credential can be verified by anyone, allowing Alice to take control of it and share it with
//! whomever they please.
//!
//! cargo run --example 5_create_vc

use iota_client::block::address::Address;
use iota_client::secret::stronghold::StrongholdSecretManager;
use iota_client::secret::SecretManager;
use iota_client::Client;

use examples::create_did;
use examples::random_stronghold_path;
use examples::API_ENDPOINT;
use identity_iota::core::json;
use identity_iota::core::FromJson;
use identity_iota::core::ToJson;
use identity_iota::core::Url;
use identity_iota::credential::Credential;
use identity_iota::credential::CredentialBuilder;
use identity_iota::credential::CredentialValidationOptions;
use identity_iota::credential::CredentialValidator;
use identity_iota::credential::FailFast;
use identity_iota::credential::Subject;
use identity_iota::crypto::KeyPair;
use identity_iota::crypto::ProofOptions;
use identity_iota::did::DID;
use identity_iota::iota::IotaDocument;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // Create a new client to interact with the IOTA ledger.
  let client: Client = Client::builder().with_primary_node(API_ENDPOINT, None)?.finish()?;

  // Create an identity for the issuer with one verification method `key-1`.
  let mut secret_manager_issuer: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password_1")
      .build(random_stronghold_path())?,
  );
  let (_, issuer_document, key_pair): (Address, IotaDocument, KeyPair) =
    create_did(&client, &mut secret_manager_issuer).await?;

  // Create an identity for the holder, in this case also the subject.
  let mut secret_manager_alice: SecretManager = SecretManager::Stronghold(
    StrongholdSecretManager::builder()
      .password("secure_password_2")
      .build(random_stronghold_path())?,
  );
  let (_, alice_document, _): (Address, IotaDocument, KeyPair) = create_did(&client, &mut secret_manager_alice).await?;

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
  let mut credential: Credential = CredentialBuilder::default()
    .id(Url::parse("https://example.edu/credentials/3732")?)
    .issuer(Url::parse(issuer_document.id().as_str())?)
    .type_("UniversityDegreeCredential")
    .subject(subject)
    .build()?;

  // Sign the Credential with the issuer's verification method.
  issuer_document.sign_data(&mut credential, key_pair.private(), "#key-1", ProofOptions::default())?;

  // Before sending this credential to the holder the issuer wants to validate that some properties
  // of the credential satisfy their expectations.

  // Validate the credential's signature using the issuer's DID Document, the credential's semantic structure,
  // that the issuance date is not in the future and that the expiration date is not in the past:
  CredentialValidator::validate(
    &credential,
    &issuer_document,
    &CredentialValidationOptions::default(),
    FailFast::FirstError,
  )
  .unwrap();

  println!("VC successfully validated");

  // The issuer is now sure that the credential they are about to issue satisfies their expectations.
  // The credential is then serialized to JSON and transmitted to the subject in a secure manner.
  let _credential_json: String = credential.to_json()?;

  println!("Credential JSON > {credential:#}");

  Ok(())
}
